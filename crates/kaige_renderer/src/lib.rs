/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// ```
// fn main() { 
//     Render::create("triangle", ("1.0", "2.0", "3.0"), "Color"); // Create, creates the window and creates a render object

//     //Do stuff with the render object, such as movement and other physics
// } 
// ```
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use wgpu::*;

struct Render {
    //TODO
    //RENDER OBJECT
    //SHOULD CONTAIN A SHAPE, SIZE, AND COLOR
    //SHOULD HAVE A FUNCTION THAT RENDERS THE SHAPE
    //SHOULD HAVE A FUNCTION THAT CHANGES THE SHAPE
    //SHOULD HAVE A FUNCTION THAT CHANGES THE SIZE
    //SHOULD HAVE A FUNCTION THAT CHANGES THE COLOR
}

struct Mesh {
    shape: String, // circle, square, triangle, etc
    //TODO
}

struct Size {
    size: (u32, u32, Option<u32>), // Handle case where size.3 == None;
    //TODO
}

struct Color {
    rgb: (u8, u8, u8),
    //TODO
}


impl Render {
    fn create(shape: Mesh, size: Size, color: Color) -> Render {
        //TODO
        //RENDERS A NEW RENDER OBJECT
        Render {
            // Initialize the fields of the Render struct here
        }
    }
}

