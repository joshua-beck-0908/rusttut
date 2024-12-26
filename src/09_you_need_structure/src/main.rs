use std::f64::consts;

struct Circle {
    diameter: f64,
}

struct Square {
    side_length: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct EquilateralTriangle {
    side_length: f64,
}

//struct RightAngleTriangle {
//    opposite: f64,
//    adjacent: f64,
//    hypotenuse: f64,
//}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

fn main() {
    let square_size = 5.0;
    let circle_size = 3.0;
    let rectangle_width = 4.0;
    let rectangle_height = 3.0;
    let triangle_side = 4.0;
    let mut area: f64;


    println!("Constructing square with side length {square_size}.");
    let shape = Square {
        side_length: square_size,
    };
    area = shape.get_area();
    println!("Area is: {area}");
    println!("Casting to rectange.");
    let shape = Rectangle::from_square(&shape);
    area = shape.get_area();
    println!("Area is: {area}");

    println!("Constructing circle with diameter {circle_size}.");
    let shape = Circle {
        diameter: circle_size,
    };
    area = shape.get_area();
    println!("Area is: {area}");
    
    println!("Constructing rectangle with size {rectangle_width}x{rectangle_height}.");
    let shape = Rectangle {
        width: rectangle_width,
        height: rectangle_height,
    };
    area = shape.get_area();
    println!("Area is: {area}");

    println!("Constructing equilateral triangle with size {triangle_side}.");
    let shape = EquilateralTriangle {
        side_length: triangle_side,
    };
    area = shape.get_area();
    println!("Area is: {area}");
    
    let shape = Triangle {
        a: 2.0, b: 3.0, c: 4.0,
    };
    // You can't be serious... no field access in formatted strings.
    //println!("Constructing triangle with side lengths: {triangle.a}, {triangle.b}, {triangle.c}");
    println!("Constructing triangle with side lengths: {}, {}, {}", shape.a, shape.b, shape.c);
    area = shape.get_area();
    println!("Area is: {area}");
}

// Original method syntax
/// How do you do polymophism???
/// I just need a general `Shape` class.
//fn calculate_rectangle_area(shape: Rectangle) -> f64 {
//    shape.height * shape.width
//}
//
//fn calculate_square_area(shape: Square) -> f64 {
//    f64::powf(shape.side_length, 2.0)
//}
//
//fn calculate_circle_area(shape: Circle) -> f64 {
//    // Is there a... math.pi or something???
//    let radius = shape.diameter / 2.0;
//    let radius_square = radius.powf(2.0);
//    radius_square * consts::PI
//}
//
//fn calculate_equilateral_triangle_area(shape: EquilateralTriangle) -> f64 {
//    let multiplier = f64::sqrt(3.0) / 4.0;
//    let square = shape.side_length.powf(2.0);
//    multiplier * square
//}
//
//fn calculate_triangle_area(shape: Triangle) -> f64 {
//    let s = (shape.a + shape.b + shape.c) / 2.0;
//    let area = s * (s - shape.a) * (s - shape.b) * (s - shape.c);
//    area.sqrt()
//}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.height * self.width
    }
    
    fn from_square(square: &Square) -> Self {
        Self {
            width: square.side_length,
            height: square.side_length,
        }
    }
}

impl Square {
    fn get_area(&self) -> f64 {
        f64::powf(self.side_length, 2.0)
    }
}

impl Circle {
    fn get_area(&self) -> f64 {
        let radius = self.diameter / 2.0;
        let radius_square = radius.powf(2.0);
        radius_square * consts::PI
    }
}

impl EquilateralTriangle {
    fn get_area(&self) -> f64 {
        let multiplier = f64::sqrt(3.0) / 4.0;
        let square = self.side_length.powf(2.0);
        multiplier * square
    }
}

impl Triangle {
    fn get_area(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2.0;
        let area = s * (s - self.a) * (s - self.b) * (s - self.c);
        area.sqrt()
    }
}
