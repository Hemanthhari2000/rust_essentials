fn area(width: i32, height: i32) -> i32 {
    width * height
}

fn area_with_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area_with_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let dimensions = (30, 50);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_tuples(dimensions)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area_with_struct(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
