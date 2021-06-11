use std::usize;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];
        println!("{:?}", v);
        println!("found it in {}", find_linear(&v, 8).unwrap());
    }

    #[test]
    fn test_bound_high() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];
        println!("{:?}", v);
        println!("found it in {}", find_binary(&v, 10).unwrap());
    }

    #[test]
    fn test_bound_low() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 10];
        println!("{:?}", v);
        println!("found it in {}", find_binary(&v, 1).unwrap());
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
        println!("{:?}", v);
        assert_eq!(find_binary(&v, 0), None);
    }

    #[test]
    fn test_empty_vector() {
        let v = vec![];
        println!("{:?}", v);
        assert_eq!(find_binary(&v, 0), None);
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
