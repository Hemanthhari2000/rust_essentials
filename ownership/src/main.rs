fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", World!");
}

fn main() {
    let s1 = String::from("Hello There");
    let len = calculate_length(&s1);

    println!("Length is {}", len);

    let mut s2 = String::from("Hello");
    change(&mut s2);
    println!("{}", s2);
}
