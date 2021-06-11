use std::usize;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];
        println!("{:?}", v);
        println!("found it in {}", find_linear(&v, 8).unwrap());
    }

    #[test]
    fn test_middle() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];
        println!("{:?}", v);
        println!("found it in {}", find_binary(&v, 5).unwrap());
    }

    #[test]
    fn test_bound_high() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];
        println!("{:?}", v);
        let found = find_binary(&v, 10).unwrap();
        println!("found it in {}", found);
        assert_eq!(found, 8);
    }

    #[test]
    fn test_bound_low() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];
        println!("{:?}", v);
        let found = find_binary(&v, 1).unwrap();
        println!("found it in {}", found);
        assert_eq!(found, 0);
    }

    #[test]
    fn test_out_of_bound_high() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];
        println!("{:?}", v);
        assert_eq!(find_binary(&v, 11), None);
    }

    #[test]
    fn test_out_of_bound_low() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];

        assert_eq!(find_binary(&v, 0), None);
    }

    #[test]
    fn test_empty_vector() {
        let v = vec![];
        println!("{:?}", v);
        assert_eq!(find_binary(&v, 0), None);
    }

    #[test]
    fn test_all_indices() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];
        for (idx, val) in v.iter().enumerate() {
            let found = find_binary(&v, *val).unwrap();
            assert_eq!(found, idx);
        }
    }

    #[test]
    fn test_all_indices_reversed() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];
        for (idx, val) in v.iter().rev().enumerate() {
            let found = find_binary(&v, *val).unwrap();
            assert_eq!(found, v.len() - idx - 1);
        }
    }
}

fn find_linear(v: &[i32], what: i32) -> Option<usize> {
    for (idx, num) in v.iter().enumerate() {
        if *num == what {
            return Some(idx);
        }
    }
    return None;
}

fn find_binary(v: &[i32], what: i32) -> Option<usize> {
    if v.is_empty() || v.first().unwrap().to_owned() > what || v.last().unwrap().to_owned() < what {
        return None;
    }
    let mut high = v.len() - 1;
    let mut low = 0;
    while low <= high {
        let mid = (low + high) / 2;
        if v[mid] == what {
            return Some(mid);
        }
        if what > v[mid] {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    return None;
}

fn main() {}
