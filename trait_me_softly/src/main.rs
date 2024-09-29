use std::fmt::Display;

trait Area {
    fn area(&self) -> f64;
}

trait Perimeter {
    fn perimeter(&self) -> f32;
}

struct Rectangle {
    width: f64,
    heigth: f64,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Width: {}; Heigth: {}", self.width, self.heigth)
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.heigth
    }
}

impl Perimeter for Rectangle {
    fn perimeter(&self) -> f32 {
        2.00 * (self.width as f32 + self.heigth as f32)
    }
}

fn main() {
    let my_rectangle: Rectangle = Rectangle {
        width: 1.1,
        heigth: 2.1,
    };

    let my_rectangle_area: f64 = my_rectangle.area();
    let my_rectangle_perimeter: f32 = my_rectangle.perimeter();

    println!("My rectangle: {}", my_rectangle);
    println!("Area of my rectangle: {}", my_rectangle_area);
    println!("Perimeter of my retangle: {}", my_rectangle_perimeter);
}
