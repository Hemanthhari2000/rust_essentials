#[derive(PartialEq, Debug)]
struct Shoe {
    size: u8,
}

fn iter_over_array() {
    let v1 = [1, 3, 4, 5, 6, 7];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("{val}");
    }
}

fn shoes_in_size(shoes: [Shoe; 3], size: u8) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == size).collect()
}

fn main() {
    iter_over_array();
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn checking_iter_map_returns_another_iter() {
        let v1 = [1, 2, 3];

        let v1_map: Vec<i32> = v1.iter().map(|x| x + 1).collect();

        assert_eq!(v1_map, [2, 3, 4]);
    }

    #[test]
    fn filter_shoes_by_size() {
        let shoes = [Shoe { size: 10 }, Shoe { size: 10 }, Shoe { size: 9 }];

        let shoes_that_fit = shoes_in_size(shoes, 10);

        assert_eq!(shoes_that_fit, [Shoe { size: 10 }, Shoe { size: 10 }])
    }
}
