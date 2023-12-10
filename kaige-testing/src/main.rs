use kaige::custom_errors::*;
use kaige::*;

#[derive(Debug, PartialEq)]
struct Health(i32);

#[derive(Debug, PartialEq)]
struct Name(&'static str);


fn main() {
    // Create a new world
    let mut world = World::new();

    // Create a new entity
    let entity = world.new_entity();
    println!("Created entity: {:?}", entity);

    // Add a Health component to the entity
    let health = Health(100);
    world.add_component_to_entity(&entity, health);

    // Access the Health component for the entity
    if let Some(component_vec) = world.borrow_component_vec_mut::<Health>() {
        if let Some(health_component) = component_vec[entity.id].as_ref() {
            println!("Health component for entity {:?}: {:?}", entity, health_component);
        } else {
            println!("Entity {:?} does not have a Health component.", entity);
        }
    } else {
        println!("No Health components found in the world.");
    };
}

// Simulate an error
fn simulate_error() -> Result<(), Errors> {
    Err(Errors::TestError)
}
