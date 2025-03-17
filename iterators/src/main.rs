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

#[cfg(test)]
mod tests {

    #[test]
    fn checking_next_returns_array_elements() {
        let v1 = [1, 2, 3];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }

    #[test]
    fn checking_iter_sum_returns_running_sum() {
        let v1 = [1, 2, 3];
        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}
