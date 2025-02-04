fn basic_enum_example() {
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let v4_addr = IpAddr::V4(String::from("127.0.0.1"));
    let v6_addr = IpAddr::V6(String::from("::1"));

    println!("{:#?}", v4_addr);
    println!("{:#?}", v6_addr);

}


fn match_pattern_example() {
    enum Coin {
        Penny, 
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin:: Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => {
                println!("Oh yeah... You have a Quarter");
                25
            },
        }
    }

    let penny = value_in_cents(Coin::Penny);
    println!("The value is {penny}");
    let nickel = value_in_cents(Coin::Nickel);
    println!("The value is {nickel}");
    let dime = value_in_cents(Coin::Dime);
    println!("The value is {dime}");
    let quarter = value_in_cents(Coin::Quarter);
    println!("The value is {quarter}");
}


fn options_with_match_example() {
    fn add_one(a: Option<i8>) -> Option<i8>{
        match a {
            None => None,
            Some(i) => Some(i+1),
        }
    }

    let six = add_one(Some(5));
    let none = add_one(None);
    println!("six: {:?}", six);
}

fn if_let_syntax_example() {
    let a = Some(4);

    match a { 
        Some(value) => println!("Value is {value}"),
        None => println!("This is None case btw."),
    }

    // Above `match` statement can be written as follows

    if let Some(value) = a {
        println!("Hey yeah, this can also be implmented like this {value}");
    }
}

fn main() {
    if_let_syntax_example();
}















