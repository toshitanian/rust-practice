mod lib;

use std::mem;

fn main() {
    let r1 = lib::Rectangle {
        name: String::from("rect 1"),
        color: Some(String::from("pink")),
        width: 10,
        height: 20,
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

    let mut r2 = lib::Rectangle {
        name: String::from("rect 2"),
        color: None,
        width: 100,
        height: 50,
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
    let r3 = Box::new(lib::Rectangle {
        name: String::from("rect 3"),
        color: None,
        width: 10,
        height: 10,
    });
    r3.show_info();

    println!("rectangle occupies {} bytes on the stack", mem::size_of_val(&r2));
    println!("Box of rectangle occupies {} bytes on the stack. This is the size of pointer.", mem::size_of_val(&r3));
    let r3_unboxed = *r3;
    println!("Unboxed rectangle occupies {} bytes on the stack. This is the size of pointer.", mem::size_of_val(&r3_unboxed));
}
