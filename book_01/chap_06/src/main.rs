fn test(a: i32, b: i32) -> bool {
    a < b
}

fn test_if_loop() {
    let a = 10;
    let b = 20;
    if test(a, b) {
        println!("a < b");
    } else if a == b {
        println!("a == b");
    } else {
        println!("a > b");
    }

    let x = if a < b {1} else {0};
    println!("{}", x);

    let v = vec![10, 20, 30, 40, 50];
    for i in v {
        println!("{}", i);
    }
    let v = vec![10, 20, 30, 40, 50];
    for mut i in v {
        i += 1;
        println!("{}", i);
    }
    let v = vec![10, 20, 30, 40, 50];
    for i in &v {
        println!("{}", i);
    }
    for (i, x) in v.iter().enumerate() {
        println!("{}, {}", i, x);
    }

    for i in 0..10 {
        if i == 5 {
            break;
        }
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }
    loop {
        println!("{}", i);
        i += 1;
        if i > 20 {
            break;
        }
    }

}

#[derive(Debug)]
enum LANG {
    JAPANESE = 81,
    ENGLISH,
    CHINESE,
    FRENCH,
}

fn test_match() {
    let lang = LANG::JAPANESE;
    println!("{:?}", lang);
    // println!("{}", lang as i32);

    let m = match lang {
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH => "英語",
        LANG::CHINESE => "中国語",
        _ => "その他",
    };
    println!("{}", m);


    let x = Option::Some(10);
    let x = Some(10);
    // let x = None;
    let v = match x {
        Some(i) => i,
        None => -1,
    };
    println!("{}", v);

    let x = Some(10);
    if let Some(i) = x {
        println!("{}", i);
    }

    let x = 3;
    let m = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("{}", m);

    let x = 11;
    let m = match x {
        0..=10 => "0 <= x <= 10",
        11..=20 => "11 <= x <= 20",
        _ => "other",
    };
    println!("{}", m);

    let c = 'A';
    match c {
        'A' => {
            println!("A");
        },
        _ => {
            println!("other");
        },
    }

    match "def" {
        "abc" => {
            println!("abc");
        },
        _ => {
            println!("other");
        },
    }

}

fn main() {
    println!("Hello, world!");
    // test_if_loop();
    test_match();

}
