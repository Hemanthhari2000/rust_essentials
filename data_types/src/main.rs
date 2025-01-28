use std::io;

fn panic_example() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array  index.");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

fn function_parameter_example() {
    fn print_labelled_data(value: i32, unit_label: char) {
        println!("The measurement is as follows: {value}{unit_label}")
    }

    print_labelled_data(5, 'h');
}

fn function_with_return_example() {
    fn get_four() -> u8 {
        4
    }

    let value = get_four();
    println!("The value is {value}");
}

fn if_statement_example() {
    let number = 1;

    if number < 9 {
        println!("The number is less than 9");
    } else {
        println!("The number is greater than 9")
    }

    let a = if number < 5 { 1 } else { 0 }; // This is possible in Rust as `if` is an expression.
    println!("The value of a is {a}");
}

fn for_loop_example() {
    for number in (1..4).rev() {
        println!("{number}...");
    }
    println!("Lift Off!!!");
}

fn main() {
    for_loop_example();
}
