#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // CONSTRUCTORS
    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }


    // METHODS
    fn clone(&self) -> Rectangle {
        Rectangle {
            width: self.width,
            height: self.height,
        }
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        rect.width < self.width && rect.height < self.height
    }

}


fn main() {
    let rect1 = Rectangle {
        width: 30.0,
        height: 50.0,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());

    let mut rect2 = rect1.clone();
    let mut rect3 = rect1.clone();

    rect2.scale(2.0);
    rect3.scale(0.5);

    println!("Does rect2 fit in rect1: {}", rect1.can_hold(&rect2));
    println!("Does rect3 fit in rect1: {}", rect1.can_hold(&rect3));


    let sq = Rectangle::square(25.0);
    println!("{:?}", sq);
}
