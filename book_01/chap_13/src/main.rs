fn main() {
    println!("{:?}", ("ABC", 10));
    let n = Option::<i32>::Some(10);
    println!("{:?}, {}", n, n.unwrap());
    let n = Result::<i32, i32>::Ok(20);
    println!("{:?}, {}", n, n.unwrap());

    println!("{a}, {b}", a = 10, b = 20);
    println!("{:x}", 200);
    println!("{:X}", 200);
    println!("{:o}", 200);
    println!("{:b}", 200);
    println!("{:4}", 200);
    println!("{:04}", 200);
    println!("{:10.5}", 123.456);
    println!("{:E}", 123.456);


    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    dbg!(a);

    #[derive(Debug)]
    struct Person {
        id: i32,
        name: String,
        age: i32,
        addr: String,
    }

    let p = Person {
        id: 10,
        name: String::from("ABC"),
        age: 20,
        addr: String::from("Tokyo"),
    };
    println!("{:?}", p);
    dbg!(p);

    struct Person2 {
        id: i32,
        name: String,
        age: i32,
        addr: String,
    }
    impl std::fmt::Debug for Person2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{ {}: `{}` in {} }}", self.id, self.name, self.addr)
        }
    }
    impl std::fmt::Display for Person2 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{ {}: `{}` in {} }}", self.id, self.name, self.addr)
        }
    }
    let p = Person2 {
        id: 10,
        name: String::from("ABC"),
        age: 20,
        addr: String::from("Tokyo"),
    };
    println!("{:?}", p);
    println!("{}", p);

    panic!("Exception occurs");
    // return;
}
