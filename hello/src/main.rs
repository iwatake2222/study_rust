struct Person {
    name: String,
    age: u32,
}

enum Event {
    Quit,
    KeyDown(u8),
    MouseDown { x: i32, y: i32 },
}



fn main() {
    let s1: String = String::from("Hello");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    let s4: &str = "aaa";

    println!("{}", s3);
    println!("{}", s4);

    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "2";
    println!("{:?}", t);

    let mut a: [i32; 3] = [0, 1, 2];
    println!("{:?}", &a[0..2]);
    let b: [i32; 3] = [0;3];

    println!("{:?}", b);

    let p = Person {
        name: String::from("John"),
        age: 8,
    };
    println!("{}", p.name);

    let e1 = Event::Quit;
    let e2 = Event::MouseDown{x: 1, y: 2};


    let result: Result<i32, String> = Ok(200);
    let result2: Result<i32, String> = Err("e".to_string());
    // if let Ok(code) = result {
    //     println!("{}", code);
    // }
    // match result {
    //     Ok(code) => println!("{}", code),
    //     Err(err) => println!("{}", err),
    // };

    println!("code: {}", result.unwrap_or(-1));
    println!("code: {}", result2.unwrap_or(-1));

    fn func(code: i32) -> Result<i32, String> {
        println!("<<<func>>>");
        println!("code: {}", code);
        Ok(100)
    }

    let result: Result<i32, String> = Ok(200);
    let result2: Result<i32, String> = Err("e".to_string());
    let next_result = result.and_then(func);
    let next_result = result2.and_then(func);

    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let code = result?;
        println!("{}", code);
        Ok(100)
    }

    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 5];
    let v3 = vec![0..5];
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", v3);
    for elem in &v1 {
        println!("{}", elem);
    }
    

    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    println!("{:?}", byte_array);
    fn my_print(s: Box<[u8]>) {
        println!("{:?}", s);
    }
    my_print(Box::new(byte_array));



    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number < 0 {
        println!("number < 0");
    } else {
        println!("number == 0");
    }

    let result = if 0 <= number {
        "positive".to_string()
    } else {
        "negative".to_string()
    };
    println!("{}", result);

    let mut count = 0;
    let result = loop {
        println!("Count: {}", count);
        count += 1;
        if count == 10 {
            break count
        }
    };
    println!("{}", result);

    let mut count = 0;
    while count < 10 {
        println!("count: {}", count);
        count += 1;
    }

    let count: i32;
    for count in 0 .. 10 {
        println!("count: {}", count);
    }

    let array = [0, 1, 2, 3, 4];
    for elem in array {
        println!("count: {}", elem);
    }

    let i: i32 = 1;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("other"),
    }

    enum Color {
        Red,
        Green,
        Blue,
    }
    let c = Color::Blue;
    match c {
        Color::Red => println!("red"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        _ => println!("misc"),
    }

    let result: Result<i32, String> = Ok(100);
    let result_number = match result {
        Ok(number) => number,
        Err(err) => {
            println!("error: {}", err);
            -1
        },
    };
    println!("{}", result_number);


    for number in 0 .. 5 {
        println!("{}", number);
    }


    struct Iter {
        current: usize,
        max: usize,
    }
    impl Iterator for Iter {
        type Item = usize;
        fn next(&mut self) -> Option<usize> {
            self.current += 1;
            if self.current - 1 < self.max {
                Some(self.current - 1)
            } else {
                None
            }
        }

    }
    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it {
        println!("{}", num);
    }

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("{}", add(1, 2));

    struct Person {
        name: String,
        age: u32,
    }
    impl Person {
        fn new(name: &str, age: u32) -> Person {
            Person {
                name: String::from(name),
                age: age,
            }
        }
        fn say_name(&self) -> &Self {
            println!("I am {}", self.name);
            self
        }
        fn say_age(&self) -> &Self {
            println!("I am {} years old", self.age);
            self
        }
    }
    let p = Person {
        name: String::from("John"),
        age: 10,
    };
    p.say_name().say_age();
    let p = Person::new("Taro", 20);
    p.say_name().say_age();

    let s = format!("{}-{:?}", "abc", ("D", 5));
    println!("{}", s);
    let s = concat!("ABC", "DEF", 3);
    println!("{}", s);

    use std::io::Write;
    let mut w = Vec::new();
    write!(&mut w, "{}", "ABC");
    println!("{:?}", w);
    eprintln!("error");
    dbg!("ddd");

    // panic!("panic");
    println!("{}", line!());
    assert!(true);
    assert_eq!(1, 1);

    // todo!();

    #[derive(Eq, PartialEq)]
    #[derive(Debug)]
    struct A(i32, i32);
    let a0 = A{0: 0, 1: 2};
    let a1 = A(0, 1);
    println!("{}", a0.0);
    println!("{}", a1.0);
    println!("{}", a0 == a1);
    println!("{:?}", a0);

    #[derive(Eq, PartialEq)]
    struct B(i32);
    let b0 = B(0);
    let b1 = B(0);
    println!("{}", b0 == b1);

}
