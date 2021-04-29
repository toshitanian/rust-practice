mod circle;
mod shape;
mod rectangle;

use std::mem;
use circle::Circle;
use rectangle::Rectangle;
use shape::Shape;

fn main() {
    let r1 = Rectangle {
        name: String::from("rect 1"),
        color: Some(String::from("pink")),
        width: 10.0,
        height: 20.0,
    };
    // Can print struct with derive(Debug)
    println!("{:?}", r1);
    // Can't use set_color method since r1 is not mutable.
    // r1.set_color(String::from("red"));

    match r1.get_color() {
        Ok(c) => println!("get_color: {}", c),
        Err(e) => println!("get_color error: {}", e),
    }
    r1.show_info();

    let mut r2 = Rectangle {
        name: String::from("rect 2"),
        color: None,
        width: 100.0,
        height: 50.0,
    };
    // Can print struct with derive(Debug)
    println!("{:?}", r2);

    match r2.get_color() {
        Ok(c) => println!("get_color: {}", c),
        Err(e) => println!("get_color error: {}", e),
    }
    r2.set_color(String::from("red"));

    r2.show_info();

    // Use box for the rect
    let r3 = Box::new(Rectangle {
        name: String::from("rect 3"),
        color: None,
        width: 10.0,
        height: 10.0,
    });
    r3.show_info();

    println!("rectangle occupies {} bytes on the stack", mem::size_of_val(&r2));
    println!("Box of rectangle occupies {} bytes on the stack. This is the size of pointer.", mem::size_of_val(&r3));
    let r3_unboxed = *r3;
    println!("Unboxed rectangle occupies {} bytes on the stack. This is the size of pointer.", mem::size_of_val(&r3_unboxed));

    // Start circle
    let circle1 = Circle::new(String::from("circle_1"), 10);
    println!("{}", circle1.area());
}
