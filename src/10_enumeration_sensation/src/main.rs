use std::f64::consts::PI;

fn main() {
    let shape = Shape::Circle { radius: 3.0 };
    shape.print_properties();
    let shape = Shape::Square { side_length: 3.0 };
    shape.print_properties();
    let shape = Shape::Rectangle { width: 3.0, height: 1.5 };
    shape.print_properties();
    let shape = Shape::EquilateralTriangle { side_length: 4.0 };
    shape.print_properties();
    let shape = Shape::ScaleneTriangle { a: 2.0, b: 3.0, c: 4.0 };
    shape.print_properties();
}


enum Shape {
    Circle { radius: f64 },
    Square { side_length: f64 },
    Rectangle { width: f64, height: f64 },
    EquilateralTriangle { side_length: f64 },
    ScaleneTriangle { a: f64, b: f64, c: f64 },
}

impl Shape {
    fn get_area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 
                PI * radius.powi(2),
            Shape::Square { side_length } => 
                side_length.powi(2),
            Shape::Rectangle { width, height } => 
                width * height,
            Shape::EquilateralTriangle { side_length } => 
                (f64::sqrt(3.0) / 4.0) * side_length.powi(2),
            Shape::ScaleneTriangle { a, b, c } => {
                let s = (a + b + c) / 2.0;
                let product = s * (s - a) * (s - b) * (s - c);
                product.sqrt()
            },
        }
    }
    
    fn get_properties_as_text(&self) -> String {
        match self {
            Shape::Circle { radius } => 
                format!("Type: Circle, Radius: {radius}"),
            Shape::Square { side_length } => 
                format!("Type: Square, Length: {side_length}"),
            Shape::Rectangle { width, height } => 
                format!("Type: Rectangle, Width: {width}, Height: {height}"),
            Shape::EquilateralTriangle { side_length } =>
                format!("Type: EquilateralTriangle, Side Length: {side_length}"),
            Shape::ScaleneTriangle { a, b, c } =>
                format!("Type: ScaleneTriangle, Side A: {a}, Side B: {b}, Side C: {c}"),
        }
    }
    
    fn print_properties(&self) {
        println!("-----------------------------");
        println!("Shape Info");
        println!("{}", self.get_properties_as_text());
        println!("Area: {}", self.get_area());
    }

}