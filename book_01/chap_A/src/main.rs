fn typename<T>(_: T)-> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    println!("=== i32 ===");
    let a: i32 = 10;
    let a_ptr: *const i32 = &a;
    println!("{:?}, {:?}, {:?}", a, a_ptr, &a as *const i32);
    println!("{}, {}", std::mem::size_of_val(&a), std::mem::size_of_val(&a_ptr));
    
    let mut a: i32 = 10;
    let a_ptr: *mut i32 = &mut a;
    unsafe {
        *a_ptr += 1;
    }
    println!("{:?}, {:?}, {:?}", a, a_ptr, &a as *const i32);

    let a: i32 = 10;
    let a_ptr: *const i32 = &a;
    println!("{:?}, {:?}", a, a_ptr);
    let b = a;
    let b_ptr: *const i32 = &b;
    println!("{:?}, {:?}", b, b_ptr);


    let a: i32 = 10;
    let a_ptr: *const i32 = &a;
    println!("{:?}, {:?}", a, a_ptr);
    let b = &a;
    let b_ptr: *const i32 = b;
    println!("{:?}, {:?}", b, b_ptr);


    println!("=== &str ===");
    let a = "0123456789";
    let a_ptr: *const &str = &a;
    let a_as_ptr = a.as_ptr();
    println!("{:?}, {:?}, {:?}", a, a_ptr, a_as_ptr);
    println!("{}, {}, {}", std::mem::size_of_val(&a), std::mem::size_of_val(&a_ptr), std::mem::size_of_val(&a_as_ptr));
    unsafe {
        let mut a_ptr = a_ptr as *const u64;
        for i in 0..std::mem::size_of_val(&a) / 8 {
            a_ptr = a_ptr.add(i);
            println!("{:?}, {:016X}", a_ptr, *a_ptr);
        }
        println!("{:?}, {:02X}", a.as_ptr(), *a.as_ptr());
    }

    println!("=== String ===");
    let a: String = "0123456789".to_string();
    let a_ptr: *const String = &a;
    let a_as_ptr = a.as_ptr();
    println!("{:?}, {:?}, {:?}", a, a_ptr, a_as_ptr);
    println!("{}, {}, {}", std::mem::size_of_val(&a), std::mem::size_of_val(&a_ptr), std::mem::size_of_val(&a_as_ptr));
    unsafe {
        let mut a_ptr = a_ptr as *const u64;
        for i in 0..std::mem::size_of_val(&a) / 8 {
            a_ptr = a_ptr.add(i);
            println!("{:?}, {:016X}", a_ptr, *a_ptr);
        }
        println!("{:?}, {:02X}", a.as_ptr(), *a.as_ptr());
    }

    println!("=== Vec ===");
    let a: Vec<i32> = vec![1, 2, 3, 4];
    let a_ptr: *const Vec<i32> = &a;
    let a_as_ptr = a.as_ptr();
    println!("{:?}, {:?}, {:?}", a, a_ptr, a_as_ptr);
    println!("{}, {}, {}", std::mem::size_of_val(&a), std::mem::size_of_val(&a_ptr), std::mem::size_of_val(&a_as_ptr));
    unsafe {
        let mut a_ptr = a_ptr as *const u64;
        for i in 0..std::mem::size_of_val(&a) / 8 {
            a_ptr = a_ptr.add(i);
            println!("{:?}, {:016X}", a_ptr, *a_ptr);
        }
        println!("{:?}, {:02X}", a.as_ptr(), *a.as_ptr());
    }

    println!("=== struct ===");
    #[derive(Debug)]
    struct Person {
        id: i32,
        age: i32,
        name: String,
    }
    let a = Person{ id: 1, age: 2, name: "ABC".to_string() };
    let a_ptr: *const Person = &a;
    let a_id_ptr: *const i32 = &a.id;
    let a_age_ptr: *const i32 = &a.age;
    let a_str_ptr: *const String = &a.name;
    println!("{:?}, {:?}, {:?}, {:?}, {:?}", a_ptr, a_id_ptr, a_age_ptr, a_str_ptr, a.name.as_ptr());
    println!("{}, {}, {}", std::mem::size_of_val(&a), std::mem::size_of_val(&a_ptr), std::mem::size_of_val(&a_id_ptr));
    unsafe {
        let mut a_ptr = a_ptr as *const u64;
        for i in 0..std::mem::size_of_val(&a) / 8 {
            a_ptr = a_ptr.add(i);
            println!("{:?}, {:016X}", a_ptr, *a_ptr);
        }
    }

    let b = a;
    let b_ptr: *const Person = &b;
    let b_id_ptr: *const i32 = &b.id;
    let b_str_ptr: *const String = &b.name;
    println!("{:?}, {:?}, {:?}, {:?}", b_ptr, b_id_ptr, b_str_ptr, b.name.as_ptr());
    unsafe {
        let mut b_ptr = b_ptr as *const u64;
        for i in 0..std::mem::size_of_val(&b) / 8 {
            b_ptr = b_ptr.add(i);
            println!("{:?}, {:016X}", b_ptr, *b_ptr);
        }
    }

    let c = &b;
    let c_ptr: *const Person = c;
    let c_id_ptr: *const i32 = &c.id;
    let c_str_ptr: *const String = &c.name;
    println!("{:?}, {:?}, {:?}, {:?}", c_ptr, c_id_ptr, c_str_ptr, c.name.as_ptr());
    unsafe {
        let mut c_ptr = c_ptr as *const u64;
        for i in 0..std::mem::size_of_val(&b) / 8 {
            c_ptr = c_ptr.add(i);
            println!("{:?}, {:016X}", c_ptr, *c_ptr);
        }
    }

    println!("\n=== typename ===");
    println!("{}, {}, {}", typename(10), typename("ABC"), typename(vec![1, 2, 3]));
    let a = Person{ id: 1, age: 2, name: "ABC".to_string() };
    println!("{}", typename(a));
}
