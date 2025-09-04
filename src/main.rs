fn main() {
    println!("Hello, Friend!");
    // A program that calculates the area of rectangle.
    // we will first start with structure, then slowly integrate 
    let width1 = 30;
    let height1 = 50;
    println!("The area of rectangle is {} sq pixels",
        area(width1,height1)
    );
    // refactoring with tuples
    let rect1 = (30,50);
    println!("The area of rectangle is {} square pixels.",
        area1(rect1)
    ) // since tuples dont have names, it will difficult if the next task is to draw the rect,
    // we wont know which is height or width
    // refactoring with structs: adding more meaning
    let rect2 = Rectangle{
        width: 30,
        height: 50
    };
    println!("The area od rectangle is {} square pixels",
        area2(&rect2)); // we sent the reference to the instance of a structure
}
struct Rectangle{
    width: u32,
    height: u32
}
fn area(width1: u32, height1: u32) -> u32{
    width1*height1
}
fn area1(dimensions: (u32, u32)) -> u32{
    dimensions.0 * dimensions.1
}
fn area2(rectangle: &Rectangle) -> u32{ //initialized into rectangle, type = immutable borrow of a struct Rectangle instance
    // we have not moved the ownership, just borrowed
    rectangle.width* rectangle.height
}/*note: accessing fields of a borrowed struct instance does not move the field values, which is
    why you often see borrows of structs. */