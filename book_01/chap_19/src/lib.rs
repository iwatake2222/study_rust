#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_add() {
        assert_eq!(add(10, 20), 30);
        assert_eq!(add(0, 20), 20);
        assert_eq!(add(-10, 20), 10);
    }
    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 10), 10);
        assert_eq!(add(10, 0), 10);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_add_str() {
        assert_eq!(add_str("ABC", "DEF"), "ABCDEF");
    }

    #[test]
    fn test() {
        assert!(true);
        assert_eq!(1, 1);
        assert_ne!(1, 0);
        let p0 = Person {id: 10, name: "ABC".to_string(), age: 20};
        let mut p1 = Person {id: 10, name: "ABC".to_string(), age: 20};
        assert_eq!(p0, p1);
        p1.age += 1;
        assert_ne!(p0, p1);

    }

}

fn add(a: i32, b: i32) -> i32 {
    let ans = a + b;
    ans
}

fn add_str(a: &str, b: &str) -> String {
    let ans = format!("{}{}", a, b);
    ans
}


#[derive(Debug, PartialEq)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}