use kaige::custom_errors::*;
use kaige::*;

fn main() {
    // Create an instance of the App struct
    let app = App::new();

    let result = simulate_error();

    match result {
        Ok(_) => println!("Operation was successful!"),
        Err(err) => match err {
            Errors::TestError => println!("Encountered a kaiGE test error!"),
            _ => println!("An unknown error occurred"),
        },
    }

    let mut world = World::new();
    let entity = world.new_entity();
    world.add_component(entity.clone(), 42);

    println!("Entity ID: {}", entity.id);
    println!("Component ID: {}", world.components[entity.id].id);
    println!(
        "Component Value: {}",
        world.borrow_component_vec_mut::<i32>().unwrap()[entity.id].unwrap()
    );

    if let Some(mut vec_ref) = world.borrow_component_vec_mut::<i32>() {
        // Do something with the mutable reference to the component vector
        vec_ref[entity.id] = Some(84);
    }

    app.run();
}

// Simulate an error
fn simulate_error() -> Result<(), Errors> {
    Err(Errors::TestError)
}
