#[derive(Debug)] // this a attribute, its suppoesed to be written directly above struct or enum
struct Rectangle{
    width: u32,
    height: u32
}
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
    ); // since tuples dont have names, it will difficult if the next task is to draw the rect,
    // we wont know which is height or width
    // refactoring with structs: adding more meaning
    let rect2 = Rectangle{
        width: 30,
        height: 50
    };
    println!("The area od rectangle is {} square pixels",
        area2(&rect2)); // we sent the reference to the instance of a structure
    // now we will try to create a method which will do the same.
    let rect3 = Rectangle{
        width: 30,
        height: 50
    };
    println!("The area of the rectange is {} sq pix", rect3.area3());
    // we are using the method synatax to call the area3 method, the method syntax is like
    // the instance then the dot followed by method name and parenthese and arguments, if any.
    println!("The value of width is valid for a rectangle: {}", rect3.width());
    // the paranthese tells Rust are we calling the method width or trying to access field width form struct
    // we can also use the methods to print out the values we want, while keeping the field private,
    // these are called getter method, when we use method to return a exisitng value,
    println!("This a getter method for height field {}", rect3.height());
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
    // the method is like a function, but we are using the keyword impl instead of fn and its connected to struture
impl Rectangle { // its called implementation block, and is associated to Rectangle type,
    fn area3(&self) -> u32 { //single parameter "self", "&self" is short for self: &self,
    // the & indicates it borrows the Self instance, the reason to chose &self is same as &rectangle, we dont want to take ownership.
    // if wanted to change the instance, we would have used &mut self.
        self.width * self.height
    }
}
// methods can also have the name of struct's field,
impl Rectangle {
    fn width(&self) -> bool{
        self.width>0
    }
}
impl Rectangle {
    fn height(&self) -> u32{
        self.height
    }
    
}