fn struct_previous_example() {
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

fn struct_basic_example() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    fn build_user(username: String, email: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }

    let user1 = build_user(String::from("clark"), String::from("clark@email.com"));
    println!("{0}", { user1.username });
}

fn struct_with_rectangle_example() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn area(rect: &Rectangle) -> u32 {
        rect.width * rect.height
    }

    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    let area = area(&rect1);
    println!("Area of {rect1:?} is {area}");
    println!("Area of {rect1:#?} is {area}");

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 10,
    };
    dbg!(&rect2);
    println!("{scale}");
    println!("{rect2:?}");

    impl Rectangle {
        fn new() -> Self {
            Self {
                width: 10,
                height: 20,
            }
        }
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let area3 = rect3.area();
    println!("Area of {rect3:#?} is {area3}");

    println!("can hold rect2 in rect1 {}", rect1.can_hold(&rect2));
    println!("can hold rect3 in rect2 {}", rect2.can_hold(&rect3));
    println!("can hold rect1 in rect3 {}", rect3.can_hold(&rect1));

    let rect4 = Rectangle::new();
    println!("{rect4:#?}");
}

fn main() {
    struct_with_rectangle_example();
}
