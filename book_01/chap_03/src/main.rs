fn add( x: i32, y : i32) -> i32 {
    x + y
}

fn test_basics() {
    let name = "abc def";
    let age : i32 = 30;
    let ans = add(3, 4);

    println!("{}, {}, {}", name, age, ans);
    let temp = b'A';
    println!("{}", temp);

    let x = 100.234;
    println!("x is {}", x);
    let x : f32 = 100.234;
    println!("x is {}", x);
    let x : f64 = 100.234;
    println!("x is {}", x);

    let f = true;
    println!("{}", f);

    let c : char = 'A';
    println!("{}", c);
    let c : char = 'あ';
    println!("{}", c);
    let s : &str = "A";
    println!("{}", s);

    let s : &str = "ABC";
    println!("{}", s);

    let s : String = String::from("ABC");
    println!("{}", s);

    let s0 : &str = "ABC";
    let s1 : &str = "DEF";
    // println!("{}", s0 + s1);
    let s = format!("{} {}", s0, s1);
    println!("{}", s);
    let s0 : String = String::from("ABC");
    let s1 : String = String::from("DEF");
    println!("{}", s0 + &s1);
    // let s = format!("{} {}", s0, s1);
    // println!("{}", s);

    let t = ("abc", 10, 20, s);
    println!("{}, {}, {}, {}", t.0, t.1, t.2, t.3);
    println!("{:?}", t);

    let arr = ["A", "B", "C"];
    let i = 1;
    println!("{}, {}, {}, {}", arr[0], arr[1], arr[2], arr[i]);
}

fn string_length(s : String) -> usize {
    let length = s.len();
    length
}

fn string_length2(s : &String) -> usize {
    let length = s.len();
    length
}

fn test_move() {
    let x = 100;
    // let x = (1, 29);
    // let x = [1, 2];
    let y = x;
    println!("{:?}", x);
    println!("{:?}", y);

    let x = String::from("ABC");
    // let y = x;
    let y = &x;
    println!("{:?}", x);
    println!("{:?}", y);

    let x = String::from("ABC");
    println!("{}", x);
    // let len = string_length(x);
    let len = string_length2(&x);
    println!("{}", len);
    println!("{}", x);
}

fn test_bind() {
    // let x = 100;
    let mut x = 100;
    x = 200;
    println!("{}", x);

    let a = 100;
    println!("{}", a);
    let a = 200;
    println!("{}", a);
}

fn test_scope() {
    let x = 100;
    println!("{}", x);
    {
        let x = 200;
        println!("{}", x);
    }
    println!("{}", x);

    let num = 10;
    let add_one = |x| { num + x };
    let add_two = |x, y| { x + y };
    let ans = add_one(1);
    println!("{}", ans);
    println!("{}", add_one(10));
    println!("{}", add_one(10));
    let ans = add_two(10, 20);
    println!("{}", ans);

    // fn func_add_one(x: i32) -> i32 {
    //     x + num
    // }
    // println!("{}", func_add_one(10));

}

fn main() {
    println!("Hello, world!");
    // test_basics();
    // test_move()
    // test_bind();
    test_scope();
}
