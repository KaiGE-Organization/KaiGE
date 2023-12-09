use kaige::custom_errors::*;
use kaige::*;

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Info {
    username: String,
}

fn main() {
    let mut world = World::default();

    // Add entities to the world
    world.push((
        Position { x: 0.0, y: 0.0 },
        Velocity { x: 1.0, y: 1.0 },
        Info { username: "tyler".to_string() }, 
    ));
    world.push((
        Position { x: 5.0, y: 5.0 },
        Velocity { x: -1.0, y: -1.0 },
        Info { username: "ashley".to_string() }, 
    ));
    world.push((
        Position { x: 10.0, y: 29.0},
        Velocity { x: 0.0, y: 0.0},
        Info { username: "brody".to_string() }, 
    ));

    // Search for all entities with values
    let mut query = <(&Info, &Position, &Velocity)>::query();

    // Iterate over each entity
    for (info, position, velocity) in query.iter(&world) {
        println!("Username: {:?}, Position: {:?}, Velocity: {:?}", info, position, velocity);
    }

    let result = simulate_error();
    
    match result {
        Ok(_) => println!("Operation was successful!"),
        Err(err) => {
            match err {
                Errors::TestError => println!("Encountered a kaiGE test error!"),
                _ => println!("An unknown error occurred"),
            }
        }
    }
}

// Simulate an error
fn simulate_error() -> Result<(), Errors> {
    Err(Errors::TestError)
}

