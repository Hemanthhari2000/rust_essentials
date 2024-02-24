
// fn immutable_variable(){
//     let bunnies = 4;
//     // bunnies = 3; // Error
//     println!("{}", bunnies);
// }

// fn mutable_variable(){
//     let mut bunnies: i32 = 32;
//     bunnies = 123; // No Error
//     println!("{}", bunnies);
// }

// fn variable_scope_1(){
//     let x = 5;
//     {
//         let y = 99;
//         println!("{}, {}", x, y);
//     }
//     println!("{}, {}", x, y); // Error 
// }

// fn variable_scope_2(){
//     let x = 5; 
//     {
//         let x = 99;
//         println!("{}", x); // Prints "99"
//     }
//     println!("{}", x) // Prints "5"
// }

// fn variable_scope_shadowing(){
//     let mut x = 5;
//     let x = x;
//     // Similarly, 
//     let meme = "More cowbell!";
//     let meme = make_image(meme);
// }


// In this example the compiler cannot guarantee that the variable `enigma` will be declared before being used.
// fn memory_safety_1() {
//     let enigma: i32;
//     println!("{}", enigma); // Error.
// }

// Even in a condition like this, the compiler still cannot understand it.
// fn memory_safety_2(){
//     let enigma: i32;
//     if true{
//         enigma = 42;
//     }
//     println!("enigma is {}", enigma); // Error
// }

// But this works, because of the compiler is sure that the value is declared before being used.
// fn memory_safety_3(){
//     let enigma: i32;
//     if true{
//         enigma = 42;
//     }
//     else {
//         enigma = 2;
//     }
//         println!("enigma is {}", enigma); // Error
// }

// Idiomatic expression to return values from a function
// fn area_of_square(side: i32) -> i32 {
//     side * side
// }

// use hello::greet;

fn say_hello(name: &mut String){
    *name = "Hello".to_string();

}


fn main() {
    //println!("Hello World");
    // greet()
    let mut name = "Hemanth".to_string();
    say_hello(&mut name);   
    println!("{}", name)

}
