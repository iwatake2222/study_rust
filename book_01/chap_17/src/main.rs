fn main() {
    let a = [1, 2, 3, 4, 5];
    let b = a.iter().map(|x| x * 2);
    println!("{:?}", b);
    for i in b {
        println!("{}", i);
    }
    let b = a.iter().filter(|&x| x % 2 == 0);
    println!("{:?}", b);
    for i in b {
        println!("{}", i);
    }
    let b = a.iter().find(|&&x| x > 3);
    println!("{:?}", b);
    for i in b {
        println!("{}", i);
    }
    println!("{:?}, {:?}", a.iter().max(), a.iter().min());

    let a = [1, 2, 3, 4, 5];
    let b = ["ABC", "DEF", "GHI"];
    let c: Vec<_> = a.iter().zip(b.iter()).collect();
    println!("{:?}", c);
    for (n, w) in c {
        println!("{}, {}", n, w);
    }

    let print_name = |name: &str, age: i32| {
        println!("{}, {}", name, age);
        age + 1
    };
    let n = print_name("ABC", 20);
    println!("{}", n);

    let a = [("ABC", 10), ("DEF", 20), ("GHI", 30)];
    let b = a.iter().map(|(name, age)| {format!("{}, {}", name, age)});
    for i in b {
        println!("{}", i);
    }


    fn call_with_one<F>(x: usize, func: F) -> usize
        where F: Fn(usize) -> usize {
        func(x)
    }

    let double = |x| x * 2;
    let a = call_with_one(100, double);
    println!("{}", a);

    fn call_with_vec<F>(v: &Vec::<usize>, func: F) -> usize
        where F: Fn(usize) -> usize {
            let mut sum = 0;
            for it in v {
                sum += func(*it);
            }
            sum
        }
    
    let double = |x| x * 2;
    let v = vec![1, 2, 3, 4, 5];
    let b = call_with_vec(&v, double);
    println!("{}", b);
}
