fn test_char() {
    let c0 = '\x41';
    let c1 = '\u{1F431}';
    println!("{}, {}", c0, c1);
    let u0 = c0 as u8;
    let u1 = c1 as u8;
    println!("{}, {}", u0, u1);

    let cc0 = u0 as char;
    let cc1 = u1 as char;
    println!("{}, {}", cc0, cc1);
}

fn test_str() {
    let s = "Hello rust world.";
    let hello = &s[0..5];
    let world = &s[11..];
    println!("{}, {}, {}", s, hello, world);
    println!("{}", s.len());
    println!("{:?}, {:?}, {:?}", s.as_ptr(), hello.as_ptr(), world.as_ptr());
    

    let mut s = String::new();
    s.push_str("Hello ");
    s.push_str("world.");
    println!("{}", s);
    let s = format!("{} {}", "Hello", "World.");
    println!("{}", s);

    let s = "Hello".to_string();
    println!("{}", s);
    let s = String::from("Hello");
    println!("{}", s);
}

fn test_jp() {
    let s = "てすと";
    println!("{}", s);
    let s = String::from("テスト");
    println!("{}", s);

    let s = "１２３４５６７８９０あいうえおかきくえこ";
    // let hello = &s[0..5];
    // let world = &s[11..];
    // println!("{}, {}, {}", s, hello, world);
    println!("{}", s.len());

    let mut v : Vec<char> = Vec::new();
    for c in s.chars() {
        v.push(c);
    }
    let v = &v[10..15];
    let mut s = String::new();
    for c in v {
        s.push(*c);
    }
    println!("{}", s);
}

fn main() {
    // test_char();
    // test_str();
    test_jp();
}
