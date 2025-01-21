fn ownership_example() {
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    fn change(s: &mut String) {
        s.push_str(", World!");
    }

    let s1 = String::from("Hello There");
    let len = calculate_length(&s1);

    println!("Length is {}", len);

    let mut s2 = String::from("Hello");
    change(&mut s2);
    println!("{}", s2);
}

fn ownership_test_inner_scope() {
    let hello = "hello";
    {
        let hello = "world!";
        println!("Hello, {hello}");
    }
    println!("The value is {hello}");
}

fn storing_data_on_heap_with_string_example() {
    let mut s = String::from("Hello");

    s.push_str(", World!");

    println!("The string is {s}");
}

fn slices_examples() {
    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("{word}");
}

fn slices_examples2() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let s = String::from("hello world");

    let word = first_word(&s);
    println!("{word}");
}

fn slices_with_string_literal_example() {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    let s = "Lorem ipsum";

    let word = first_word(&s);
    println!("{word}");
}

fn slices_with_array_example() {
    let a = [1, 3, 4, 5, 6];

    let sub_array = &a[1..8]; // Error here as the index is out of range
    println!("{:?}", sub_array);
}

fn main() {
    slices_with_array_example();
}
