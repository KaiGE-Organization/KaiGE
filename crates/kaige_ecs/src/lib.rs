use std::cell::{RefCell, RefMut};
#[derive(Debug, PartialEq)]
struct Health(i32);

#[derive(Debug, PartialEq)]
struct Name(&'static str);

#[derive(Debug, PartialEq)]
pub struct Entity {
    pub id: usize,
}

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}

impl<T: 'static> ComponentVec for RefCell<Vec<Option<T>>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.get_mut().push(None)
    }
}

pub struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        Entity { id: entity_id }
    }

    pub fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: &Entity,
        component: ComponentType,
    ) {
        // Search for any existing ComponentVecs that match the type of the component being added.
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<ComponentType>>>>()
            {
                component_vec.borrow_mut()[entity.id] = Some(component);
                return;
            }
        }

        // No matching component storage exists yet, so we have to make one.
        let mut new_component_vec: Vec<Option<ComponentType>> =
            Vec::with_capacity(self.entities_count);

        // All existing entities don't have this component, so we give them `None`
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        // Give this Entity the Component.
        new_component_vec[entity.id] = Some(component);
        self.component_vecs
            .push(Box::new(RefCell::new(new_component_vec)));
    }

    pub fn borrow_component_vec_mut<ComponentType: 'static>(
        &self,
    ) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<ComponentType>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_entity() {
        let mut world = World::new();
        let entity = world.new_entity();
        assert_eq!(entity.id, 0);
    }

    #[test]
    fn test_add_component_to_entity() {
        let mut world = World::new();
        let entity = world.new_entity();
        let health = Health(100);
        world.add_component_to_entity(&entity, health);

        let component_vec = world.borrow_component_vec_mut::<Health>().unwrap();
        assert_eq!(component_vec[entity.id], Some(Health(100)));
    }

    #[test]
    fn test_add_component_to_entity_multiple_types() {
        let mut world = World::new();
        let entity = world.new_entity();
        let health = Health(100);
        let name = Name("John Doe");
        world.add_component_to_entity(&entity, health);
        world.add_component_to_entity(&entity, name);

        let health_component_vec = world.borrow_component_vec_mut::<Health>().unwrap();
        assert_eq!(health_component_vec[entity.id], Some(Health(100)));

        let name_component_vec = world.borrow_component_vec_mut::<Name>().unwrap();
        assert_eq!(name_component_vec[entity.id], Some(Name("John Doe")));
    }

    #[test]
    fn test_add_component_to_entity_no_matching_component_vec() {
        let mut world = World::new();
        let entity = world.new_entity();
        let name = Name("John Doe");
        world.add_component_to_entity(&entity, name);

        let name_component_vec = world.borrow_component_vec_mut::<Name>().unwrap();
        assert_eq!(name_component_vec[entity.id], Some(Name("John Doe")));
    }
}