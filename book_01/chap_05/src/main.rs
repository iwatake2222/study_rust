fn add(a: i32, b: i32) -> i32 {
    println!("{}", a + b);
    a + b
}

fn modify_add(a: &mut i32) -> i32 {
    *a += 1;
    println!("{}", a);
    *a
}

fn str_param(s: &str) {
    println!("{}", s);
    // s = "def";
    println!("{}", s);
}
fn str_param_and_return(s: &str) -> String {
    let ret: String = format!("Modify: {}", s);
    ret
}

fn main() {
    let a = 10 + 20;
    let b = { 20 + 30 };
    println!("a is {}, b is {}", a, b);
    println!("{}", 10.0 % 20.0);
    let mut a = 10;
    a += 1;
    // a++;
    println!("a is {}", a);

    let a : u8 = 0b1111;
    let b : u8 = 0b0001;
    println!("{:08b}, {:08b}, {:08b}", a + b, a & b, a | b);
    println!("{:016b}", a << 7);

    add(10, 20.1 as i32);

    let mut a = 10;
    modify_add(&mut a);
    println!("{}", a);

    let s: &str = "abc";
    str_param(s);
    println!("{}", s);

    let s = str_param_and_return(s);
    println!("{}", s);
    println!("{}", str_param_and_return("DEF"));



    let v = vec![1, 2, 3, 4, 5];
    println!("{}", v[0]);
    let sum = vec_param(&v);
    println!("{}", sum);
    println!("{}", v[0]);
    fn vec_param(v: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in v {
            sum += i;
        }
        sum
    }

    fn vec_make() -> Vec<i32> {
        let mut v = Vec::new();
        v.push(10);
        v.push(20);
        v
    }
    println!("{:?}", vec_make());

    fn vec_modify(v: &mut Vec<i32>) {
        for i in v {
            *i += 1;
        }
    }

    let mut v = vec_make();
    vec_modify(&mut v);
    for i in v {
        println!("{}", i);
    }

}
