#[derive(Debug)]
struct Rectangle {
    name: String,
    color: Option<String>,
    width: i32,
    heigh: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        return self.width * self.heigh;
    }
    fn show_info(&self) {
        let col = match &self.color {
            Some(x) => x,
            None => "N/A",
        };
        println!("name: {}, color: {}, area: {}", self.name, col, self.area())
    }

    fn set_color(&mut self, color: String) {
        self.color = Some(color);
    }

    fn get_color(&self) -> Result<String, String>{
        match &self.color {
            Some(x) => return Ok(x.to_string()),
            None => return Err(String::from("color is N/A.")),
        };
    }
}
fn main() {
    let r1 = Rectangle {
        name: String::from("rect 1"),
        color: Some(String::from("pink")),
        width: 10,
        heigh: 20,
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
        width: 100,
        heigh: 50,
    };
    // Can print struct with derive(Debug)
    println!("{:?}", r2);

    match r2.get_color() {
        Ok(c) => println!("get_color: {}", c),
        Err(e) => println!("get_color error: {}", e),
    }
    r2.set_color(String::from("red"));

    r2.show_info();
}
