// fn largest_number(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for val in list {
//         if val > largest {
//             largest = val;
//         }
//     }
//     largest
// }

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

// Both x and y are of same type.
// struct Point {
//     x: T,
//     y: T,
// }

// x and y are different types
// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

struct Point<A, B> {
    x: A,
    y: B,
}

impl<A, B> Point<A, B> {
    fn mixup<C, D>(self, other: Point<C, D>) -> Point<A, D> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    // let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    // let largest = largest_number(&list);
    // println!("Largest number is {}", largest);

    // let integer = Point { x: 5, y: 7 };
    // let float = Point { x: 1.5, y: 3.5 };

    // let numbers = Point { x: 1, y: 1.5 };

    // let p = Point { x: 1, y: 2 };
    // println!("{}", p.x());

    // let q = Point { x: 1.4, y: 2.5 };
    // println!("{}", q.distance_from_origin());

    let a = Point { x: 1, y: 2 };
    let b = Point { x: "Hello", y: 'w' };
    let mixup = a.mixup(b);
    println!("{} and {}", mixup.x, mixup.y);
}
