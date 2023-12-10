use std::cell::{RefCell, RefMut};
use std::any::Any;

#[derive(Clone)]
pub struct Entity {
    pub id: usize,
}

pub struct Component {
    pub id: usize,
    // Add actual component fields here
}

impl Entity {
    fn new(id: usize) -> Self {
        Entity { id }
    }

    fn add_component(&mut self, component_id: usize) {
        // Logic to add component to entity
        self.id = component_id; // For demonstration purposes, store the component ID in the entity
    }

    fn get_component<T: 'static>(&self) -> Option<&'static T> {
        // Logic to get component from entity
        if let Some(component) = get_component_by_id::<T>(self.id) {
            Some(component)
        } else {
            None
        }
    }
}

fn get_component_by_id<T: 'static>(component_id: usize) -> Option<&'staticT> {
    // Function to retrieve the component from the World's component vectors
    // You can use the component_id to index into the appropriate vector
    // and return the component if it exists.
    // Implement this based on your specific requirements.
    unimplemented!()
}

impl Component {
    fn new(id: usize) -> Self {
        Component { id }
    }
}

pub struct World {
    pub entities: Vec<Entity>,
    pub components: Vec<Component>,
    pub component_vecs: Vec<Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: Vec::new(),
            component_vecs: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let entity = Entity::new(self.entities.len());
        self.entities.push(entity.clone()); // Clone the entity to avoid ownership issues
        entity
    }

    // Adds a component to an entity
    pub fn add_component<T: 'static>(&mut self, mut entity: Entity, component: T) {
        let component_id = self.components.len();
        self.components.push(Component::new(component_id));
        entity.add_component(component_id);

        // Ensure the component vector exists and has enough capacity
        while self.component_vecs.len() <= component_id {
            self.component_vecs.push(Box::new(RefCell::new(Vec::<Option<T>>::new())));
        }

        // Access the component vector and insert the component
        if let Some(component_vec) = self.component_vecs[component_id]
            .downcast_ref::<RefCell<Vec<Option<T>>>>()
        {
            let mut vec_ref = component_vec.borrow_mut();
            vec_ref.resize_with(self.entities.len(), || None);
            vec_ref[entity.id] = Some(component);
        }
    }

    // Borrows a component vector mutably
    pub fn borrow_component_vec_mut<T: 'static>(&self) -> Option<RefMut<Vec<Option<T>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec
                .downcast_ref::<RefCell<Vec<Option<T>>>>()
            {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }
}
