/*
Let’s make a new binary project with Cargo called rectangles
that will take the width and height of a rectangle specified
in pixels and calculate the area of the rectangle.
*/
/* -------------------- First Attempt -------------------- */
/*
fn main() {
    let width: u64 = 30;
    let height: u64 = 50;

    println!(
        "The are of rectangle is {} square pixels.",
        area(width, height)
    );
}

fn area(width: u64, height: u64) -> u32 {
    (width * height).try_into().unwrap()
}
*/

/* -------------------- Second Attempt -------------------- */

// Group width and height together using tuple
/*
fn main() {
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} squre pixels.", area(rect1));
}

fn area(dimensions: (u32, u32)) -> i32 {
    (dimensions.0 * dimensions.1).try_into().unwrap()
}
*/

/* -------------------- Third Attempt -------------------- */
// Refactoring with Structure: Adding More Meaning
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    dbg!(rectangle.width * rectangle.height)
}
