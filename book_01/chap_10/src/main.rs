fn test_ownership() {
    #[derive(Debug)]
    struct Person {
        name: &'static str,
        age: i32,
    }
    
    fn print_p(a: &Person) {
        println!("{:?}", a);
    }
    
    fn move_p(a: Person) {
        println!("{:?}", a);
    }
    
    fn add_age(p: &mut Person){
        p.age += 1;
    }
    
    let p = Person {
        name: "ABC",
        age: 10
    };
    
    print_p(&p);
    println!("{:?}", p);
    move_p(p);
    // println!("{:?}", p);

    let p0 = Person {
        name: "ABC",
        age: 10
    };
    let p1 = &p0;
    let p2 = p1;
    print_p(&p2);
    print_p(&p0);
    print_p(&p1);

    let mut p0 = Person {
        name: "ABC",
        age: 10
    };
    add_age(&mut p0);
    print_p(&p0);

    let p0 = Person {
        name: "ABC",
        age: 10
    };
    let mut p1 = p0;
    add_age(&mut p1);
    // add_age(&mut p0);
    // print_p(&p0);
    print_p(&p1);
    
    let mut p0 = Person {
        name: "ABC",
        age: 10
    };
    let mut p1 = &mut p0;
    add_age(&mut p1);
    print_p(&p1);
}

fn test_number() {
    #[derive(Debug)]
    struct Person {
        name: &'static str,
        age: i32,
    }
    let a = Person {
        name: "ABC",
        age: 10
    };
    let b = a;
    
    // println!("{:?}", a);
    println!("{:?}", b);

    let a = 10;
    let b = a;
    println!("{:?}", a);
    println!("{:?}", b);

    let a = (100, "ABC");
    let b = a;
    println!("{:?}", a);
    println!("{:?}", b);
}

fn test_string() {
    let a = vec!["abc", "def", "ghi"];
    let b = a;
    // println!("{:?}", a);
    println!("{:?}", b);

    let a = vec!["abc", "def", "ghi"];
    let b = &a;
    println!("{:?}", a);
    println!("{:?}", b);

    let a = String::from("ABCD");
    let b = &a;
    println!("{:?}", a);
    println!("{:?}", b);
}

fn test_conflict() {
    #[derive(Debug)]
    struct Person {
        name: &'static str,
        age: i32,
    }
    let a = Person {
        name: "ABC",
        age: 10
    };

    let b = &a;
    println!("{:?}", a);
    println!("{:?}", b);

    let c = a;
    // println!("{:?}", a);
    // println!("{:?}", b);
    println!("{:?}", c);
}

fn test() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: i32,
    }

    let x: &Person;
    let y: Person;
    {
        let a = Person {
            name: String::from("ABC"),
            age: 10
        };
        
        x = &a;
        println!("{:?}", x);
        y = a;
        println!("{:?}", y);
    }
    // println!("{:?}", x);
    println!("{:?}", y);
    
    let mut a = Person {
        name: String::from("ABC"),
        age: 10
    };

    let x = &mut a;
    // a.age += 1;
    x.age += 1;
    println!("{:?}", x);
    let y = &mut a;
    // println!("{:?}", x);
    // x.age += 1;
    y.age += 1;
    println!("{:?}", y);
    
    

}

fn main() {
    // test_ownership();
    // test_number();
    // test_string();
    // test_conflict();
    test();
}
