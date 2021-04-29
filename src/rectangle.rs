
use crate::shape::Shape;

#[derive(Debug)]
pub struct Rectangle {
    pub name: String,
    pub color: Option<String>,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {

    pub fn show_info(&self) {
        let col = match &self.color {
            Some(x) => x,
            None => "N/A",
        };
        println!("name: {}, color: {}, area: {}", self.name, col, self.area())
    }

    pub fn set_color(&mut self, color: String) {
        self.color = Some(color);
    }

    pub fn get_color(&self) -> Result<String, String>{
        match &self.color {
            Some(x) => return Ok(x.to_string()),
            None => return Err(String::from("color is N/A.")),
        };
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}