fn test_struct() {
    struct Person {
        id: i32,
        name: String,
        age: i32,
        addr: String,
    }
    
    static mut PERSON_ID: i32 = 0;
    impl Person {
        fn new(name: &str, age: i32, addr: &str) -> Person {
            let id = unsafe {
                PERSON_ID += 1;
                PERSON_ID
            };
            let pa = Person {
                id: id,
                name: name.to_string(),
                age: age,
                addr: addr.to_string(),
            };
            pa
        }
        fn print(&self, private: bool) {
            if private {
                println!("{}", self.name);
            } else {
                println!("{}, {}, {}, {}", self.id, self.name, self.age, self.addr);
            }
        }
    
        fn to_str(&self) -> String {
            let s = format!("{}", self.name);
            s
        }
    
        fn add_age(&mut self) {
            self.age += 1;
        }
    }
    
    fn print_person(pa: &Person) {
        println!("{}, {}, {}, {}", pa.id, pa.name, pa.age, pa.addr);
    }
    
    fn add_age_person(pa: &mut Person) {
        pa.age += 1;
    }
    
    fn new_person(id: i32, name: &str) -> Person {
        let pa = Person {
            id: id,
            name: String::from(name),
            age: 50,
            addr: String::from("Tokyo"),
        };
        pa
    }



    let mut pa = Person {
        id: 100,
        name: String::from("ABC"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    println!("{}, {}, {}, {}", pa.id, pa.name, pa.age, pa.addr);
    pa.id = 200;
    println!("{}, {}, {}, {}", pa.id, pa.name, pa.age, pa.addr);
    print_person(&pa);
    add_age_person(&mut pa);
    print_person(&pa);

    let pa2 = new_person(500, "DEF");
    print_person(&pa2);

    let mut v: Vec<Person> = Vec::new();
    v.push(pa);
    v.push(pa2);
    for p in &v {
        print_person(p);
    }

    struct Color(f32, f32, f32);
    let yellow = Color(1.0, 1.0, 0.0);
    println!("{}, {}, {}", yellow.0, yellow.1, yellow.2);

    println!("{}, {}", std::mem::size_of::<Person>(), std::mem::size_of::<Color>());

    let mut pa = new_person(100, "ABC");
    pa.print(true);
    pa.print(false);
    println!("{}", pa.to_str());
    pa.add_age();
    pa.print(false);

    let mut pa = Person::new("GHI", 20, "OSAKA");
    pa.print(false);

}

fn test_result() {
    let r = "100agaa".parse::<i32>();
    println!("{:?}", r);
    match r {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("err is {:?}", e),
    }
    let r = "100".parse::<i32>().unwrap();
    println!("{:?}", r);


    fn half_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        match s.parse::<i32>() {
            Ok(n) => Ok(n / 2),
            Err(e) => Err(e)
        }
    }

    let n = half_number("10");
    match n {
        Ok(n) => println!("{}", n),
        Err(e) => println!("err: {:?}", e),
    };
    let n = half_number("xxx");
    match n {
        Ok(n) => println!("{}", n),
        Err(e) => println!("err: {:?}", e),
    };

    type Result2<T> = std::result::Result<T, std::num::ParseIntError>;
    fn half_number2(s: &str) -> Result2<i32> {
        match s.parse::<i32>() {
            Ok(n) => Ok(n / 2),
            Err(e) => Err(e)
        }
    }
    let n = half_number2("10");
    match n {
        Ok(n) => println!("{}", n),
        Err(e) => println!("err: {:?}", e),
    };


    fn half_number3(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse::<i32>().map(|n| n / 2)
    }
    let n = half_number3("10");
    match n {
        Ok(n) => println!("{}", n),
        Err(e) => println!("err: {:?}", e),
    };
    fn half_number4(s: &str) -> Result<i32, std::num::ParseIntError> {
        let n = s.parse::<i32>()?;
        Ok(n / 2)
    }
    let n = half_number4("xxx");
    match n {
        Ok(n) => println!("{}", n),
        Err(e) => println!("err: {:?}", e),
    };

    fn half_number5(s: &str) -> Result<i32, &str> {
        match s.parse::<i32>() {
            Ok(n) => Ok(n / 2),
            Err(_) => Err("error"),
        }
    }
    let n = half_number5("xxx");
    match n {
        Ok(n) => println!("{}", n),
        Err(e) => println!("err: {:?}", e),
    };

    let n = "100".parse::<i32>().expect("not a number");
    println!("{}", n);
    let n = "xxx".parse::<i32>().expect("not a number");
    println!("{}", n);

}

fn main() {
    // test_struct();
    test_result();
}
