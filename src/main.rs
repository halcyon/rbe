use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;

        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }
        write!(f, "]")
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}", self.red, self.green, self.blue, self.red, self.green, self.blue)
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue:0 },
    ].iter() {
        println!("{}", *color)
    }

}
