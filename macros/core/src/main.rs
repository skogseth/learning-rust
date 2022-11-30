use declarative_macros::*;
use procedural_macros::*;

fn main() {
    let rotate = |z: Complex| z.multiply(Complex { a: 0., b: 1. });
    let z = pipes!{ Complex { a: 1., b: 0. } => rotate => rotate => rotate };
    println!("z: {:#?}", z);

    let a = pass_through!(3+4);
    println!("a = {a}");
    assert_eq!(a, 7);

    for enum_element in Hands::enumerator() {
        println!("{:?}", enum_element);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    a: f64,
    b: f64,
}

impl Complex {
    pub fn multiply(self, other: Complex) -> Self {
        Complex {
            a: self.a * other.a - self.b * other.b,
            b: self.a * other.b + self.b * other.a,
        }
    }
}

#[derive(Debug, Enumerator)]
enum Hands {
    Left,
    Right,
}
