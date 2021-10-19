fn test_array() {
    let arr = [1, 2, 3, 4, 5];
    println!("{}, {}, {}", arr[0], arr[4], arr.len());
    for i in 0 .. arr.len() {
        print!("{} ", arr[i]);
    }

    let arr = ["ABC", "DEF", "GH"];
    for i in 0 .. arr.len() {
        print!("{} ", arr[i]);
    }

    let arr : [i32; 5] = [1, 2, 3, 4, 5];
    for i in 0 .. arr.len() {
        print!("{} ", arr[i]);
    }

    let mut arr: [i32; 5] = [-1; 5];
    arr[1] = 20;
    for i in 0 .. arr.len() {
        print!("{} ", arr[i]);
    }
    
    let a = [1u8, 2u8, 3u8, 4u8];
    unsafe {
        let b = std::mem::transmute::<[u8; 4], u32>(a);
        println!("{:X}", b);
    }
    let a: u32 = 0x12345678;
    unsafe {
        let b = std::mem::transmute::<u32, [u8; 4]>(a);
        println!("{:?}", b);
    }
}

fn test_vector() {
    let v = vec![1, 2, 3, 4, 5];
    println!("{}, {}", v[0], v.len());
    println!("{:?}, {:?}", v.get(0), v.get(10));
    println!("{}, {}", v.get(0).unwrap(), v.get(1).unwrap());

    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("{:?}, {:?}", v.first(), v.last());
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());

    let mut v = vec![1, 2, 3, 4, 5];
    v.remove(2);
    for i in v {
        print!("{}  ", i);
    }
    println!();
    let mut v = vec![1, 2, 3, 4, 5];
    v.insert(0, 99);
    for i in v {
        print!("{}  ", i);
    }

    let a = vec![1, 2, 3];
    let b = vec![4, 5];
    println!("{:?}", [a, b]);

    let a = vec![1, 2, 3];
    let b = vec![4, 5];
    println!("{:?}", [a, b].concat());

    let mut v = vec!["ABC", "DEF", "GHI"];
    v.reverse();
    let x = v.join("-");
    println!("{}", x);
    let v = x.split('-');
    println!("{:?}", v);
    for i in v {
        print!("{} ", i);
    }
    
    println!();println!();

    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        print!("{} ", i);
    }
    for i in v.iter() {
        print!("{} ", i);
    }

    let mut p = v.iter();
    println!("{:?}", p);
    print!(" {:?} ", p.next());
    print!(" {:?} ", p.next());
    print!(" {:?} ", p.next());
    print!(" {:?} ", p.next());
    print!(" {:?} ", p.next());
    print!(" {:?} ", p.next());


    println!();

    let mut p = v.iter();
    while let Some(x) = p.next() {
        println!("{}", x);
    }

    let lst = v.iter().map(|x| x * 10);
    println!("{:?}", lst);


}

fn main() {
    // test_array();
    test_vector();
}
