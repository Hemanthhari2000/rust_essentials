use std::{
    collections::{HashMap, HashSet},
    io,
};

fn return_mode() {
    let v: Vec<i32> = vec![1, 2, 3, 2, 3, 2, 3, 1, 3, 4, 2, 5];
    let mut mode_map = HashMap::new();

    for i in &v {
        let mode_count = mode_map.entry(i).or_insert(0);
        *mode_count += 1;
    }
    println!("{mode_map:?}")
}

fn return_median() {
    // let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let length_of_array = v.len();
    let mut median = 0;
    let mid = length_of_array / 2;

    if length_of_array % 2 == 0 {
        median = (v[mid - 1] + v[mid]) / 2
    } else {
        median = v[mid];
    }
    println!("{median}");
}

fn pig_latin() {
    let vowels: HashSet<char> = ['a', 'e', 'i', 'o', 'u'].into_iter().collect();
    let mut word = String::from("first");
    if let Some(first_char) = word.chars().next() {
        if vowels.contains(&first_char) {
            word.push_str("-hay");
        } else {
            let rest = &word[1..];
            word = format!("{}-{}ay", rest, first_char);
        }
    }
    println!("{word}");
    println!("{vowels:?}");
}

fn team_map() {
    let mut team_mappings: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut user_command = String::new();

        println!("user command: ");

        io::stdin()
            .read_line(&mut user_command)
            .expect("Failed to read line");

        let command_words: Vec<&str> = user_command.split_whitespace().collect();
        match command_words.as_slice() {
            ["Add", emp_name, "to", team] => team_mappings
                .entry(team.to_string())
                .or_insert_with(Vec::new)
                .push(emp_name.to_string()),
            ["exit"] => break,
            _ => println!("Sorry, invalid user command"),
        }
        println!("{team_mappings:?}");
    }
}

fn main() {
    team_map();
}
