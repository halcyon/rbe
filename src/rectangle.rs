#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

// fn main() {
//     let name = "Peter";
//     let age = 27;
//     let peter = Person { name, age };

//    println!("{:?}", peter);

//     let point: Point = Point { x: 0.3, y: 0.5 };

//     println!("point coordinates: ({}, {})", point.x, point.y);

//     let new_point = Point { x: 0.1, ..point };

//     println!("second point: ({}, {})", new_point.x, new_point.y);

//     let _rectangle = Rectangle {
//         p1: Point { x: 0.3, y: 0.5 },
//         p2: Point { x: 0.5, y: 0.3 },
//     };

//     println!("area: {:.2}", rect_area(_rectangle));
//     println!("square: {:?}", square(point, 0.5));
// }


fn rect_area(rect: Rectangle) -> f32 {
    let width = (rect.p1.x - rect.p2.x).abs();
    let height = (rect.p1.y - rect.p2.y).abs();
    width * height
}

fn square(point: Point, float: f32) -> Rectangle {
    let Point { x: my_x, y: my_y } = point;
    Rectangle{ p1: point,
               p2: Point{ x: my_x + float, y: my_y + float}
    }
}
