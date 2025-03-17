fn iter_over_array() {
    let v1 = [1, 3, 4, 5, 6, 7];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("{val}");
    }
}
fn main() {
    iter_over_array();
}
