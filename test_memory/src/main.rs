fn main() {
    println!("\n####################\n# Pointer\n####################");
    let val: i32 = 0x12345678;
    let val_ptr: *const i32 = &val;
    println!("val = 0x{:08X}, val_ptr = {:?}", val, val_ptr);
    println!("sizeof(val) = {}, sizeof(val_ptr) = {}", std::mem::size_of_val(&val), std::mem::size_of_val(&val_ptr));
    
    println!("\n### Pointer Access ###");
    let mut val: i32 = 0x12345678;
    let val_ptr: *mut i32 = &mut val;
    println!("val = 0x{:08X}, val_ptr = {:?}", val, val_ptr);
    unsafe {
        *val_ptr += 1;
        println!("*val_ptr + 1 = 0x{:08X}", val);
        println!("val_ptr.add(1) = {:?}", val_ptr.add(1));
        // println!("val_ptr + 1 = {:?}", val_ptr + (1 as *mut i32));  // Error:  cannot add `*mut i32` to `*mut i32`
    }

    println!("\n### Pointer Cast ###");
    let mut val: i32 = 0x12345678;
    let val_ptr: *mut i32 = &mut val;
    let val_u8_ptr: *mut u8 = val_ptr as *mut u8;
    println!("val = 0x{:08X}, val_ptr = {:?}, val_u8_ptr = {:?}", val, val_ptr, val_u8_ptr);
    unsafe {
        for i in 0..4 {
            println!("val_u8_ptr.add({}) = {:?}, val = 0x{:02X}", i, val_u8_ptr.add(i), *val_u8_ptr.add(i));
        }
    }

    //// Exit: STATUS_ACCESS_VIOLATION
    // println!("\n### Pointer Cast from Number ###");
    // unsafe {
    //     /* immutable */
    //     let ptr: *const u32 = 0x0000_0000_0000_0000 as *const u32;
    //     println!("{}", *ptr);

    //     /* mutable */
    //     let ptr: *mut u32 = 0x0000_0000_0000_0000 as *mut u32;
    //     *ptr = 0xdeadbeef;
    // }

    println!("\n####################\n# i32\n####################");
    println!("\n### i32: Memory Allocation ###");
    let val: i32 = 0x12345678;
    let val_ptr: *const i32 = &val;
    let val_ref: &i32 = &val;
    let val_ref_ptr: *const &i32 = &val_ref;
    
    println!("val = 0x{:08X}, val_ptr = {:?}, val_ref = 0x{:08X}, val_ref_ptr = {:?}", val, val_ptr, val_ref, val_ref_ptr);
    println!("sizeof(val) = {}, sizeof(val_ptr) = {}, sizeof(val_ref) = {}, sizeof(val_ref_ptr) = {}",
        std::mem::size_of_val(&val), std::mem::size_of_val(&val_ptr), std::mem::size_of_val(&val_ref), std::mem::size_of_val(&val_ref_ptr));

    unsafe {
        println!("val_ptr = 0x{:016X}, *val_ptr = 0x{:016X}", val_ptr as usize, *val_ptr);
        println!("val_ref = 0x{:016X}, *val_ref = 0x{:016X}", val_ref, *val_ref); // get same result
        println!("val_ref_ptr = {:?}, *val_ref_ptr = {:?}", val_ref_ptr, *val_ref_ptr); // *val_ref_ptr becomes val (not val_ref = val_ptr)
        let val_ref_ptr = val_ref_ptr as *const usize;
        println!("val_ref_ptr = {:?}, *val_ref_ptr = 0x{:016X}", val_ref_ptr, *val_ref_ptr);
    }


    println!("\n### i32: Move ###");
    let val: i32 = 0x12345678;
    let val_ptr: *const i32 = &val;
    println!("Before Move");
    println!("val = 0x{:08X}, val_ptr = {:?}", val, val_ptr);

    let val_move = val;
    let val_move_ptr: *const _ = &val_move;
    println!("After Move");
    println!("val = 0x{:08X}, val_ptr = {:?}", val, val_ptr);
    println!("val_move = 0x{:08X}, val_move_ptr = {:?}", val_move, val_move_ptr);



    println!("\n####################\n# array\n####################");
    println!("\n### array: Memory Allocation ###");
    let val: [i32; 5] = [0x10, 0x20, 0x30, 0x40, 0x50];
    let val_ptr: *const _ = &val;
    let val_ref: _ = &val;
    let val_ref_ptr: *const _ = &val_ref;
    let val_as_ptr: *const _ = val.as_ptr();
    println!("val = {:?}, val_ptr = {:?}, val_ref = {:?}, val_ref_ptr = {:?}, val_as_ptr = {:?}", val, val_ptr, val_ref, val_ref_ptr, val_as_ptr);
    println!("sizeof(val) = {}, sizeof(val_ptr) = {}, sizeof(val_ref) = {}, sizeof(val_ref_ptr) = {}",
        std::mem::size_of_val(&val), std::mem::size_of_val(&val_ptr), std::mem::size_of_val(&val_ref), std::mem::size_of_val(&val_ref_ptr));
    unsafe {
        println!("== Display val_ptr ==");
        let val_u32_ptr = val_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val) / std::mem::size_of::<u32>() {
            println!("val_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_u32_ptr.add(i), *val_u32_ptr.add(i));
        }
        println!("== Display val.as_ptr() ==");
        let val_as_ptr = val.as_ptr();
        for i in 0..val.len() {
            println!("val_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_as_ptr.add(i), *val_as_ptr.add(i));
        }

        println!("==Display reference==");
        let val_ref_ptr = val_ref_ptr as *const usize;
        println!("val_ref_ptr = {:?}, *val_ref_ptr = 0x{:016X}", val_ref_ptr, *val_ref_ptr);
    }

    println!("\n### array: Move ###");
    println!("Before Move");
    let val: [i32; 5] = [0x10, 0x20, 0x30, 0x40, 0x50];
    let val_ptr: *const _ = &val;
    println!("val = {:?}, val_ptr = {:?}, val.as_ptr() = {:?}", val, val_ptr, val.as_ptr());

    println!("After Move");
    let val_move = val;
    let val_move_ptr: *const _ = &val_move;
    println!("val = {:?}, val_ptr = {:?}, val.as_ptr() = {:?}", val, val_ptr, val.as_ptr());
    println!("val_move = {:?}, val_move_ptr = {:?}, val_move.as_ptr() = {:?}", val_move, val_move_ptr, val_move.as_ptr());
    println!("sizeof(val) = {}, sizeof(val_move) = {}", std::mem::size_of_val(&val), std::mem::size_of_val(&val_move));



    println!("\n####################\n# vec\n####################");
    println!("\n### vec: Memory Allocation ###");
    let val: Vec<i32> = vec![0x10, 0x20, 0x30, 0x40, 0x50];
    let val_ptr: *const Vec<i32> = &val;
    let val_ref: &Vec<i32> = &val;
    let val_ref_ptr: *const &Vec<i32> = &val_ref;
    let val_as_ptr: *const i32 = val.as_ptr();
    println!("val = {:?}, val_ptr = {:?}, val_ref = {:?}, val_ref_ptr = {:?}, val_as_ptr = {:?}", val, val_ptr, val_ref, val_ref_ptr, val_as_ptr);
    println!("sizeof(val) = {}, sizeof(val_ptr) = {}, sizeof(val_ref) = {}, sizeof(val_ref_ptr) = {}",
        std::mem::size_of_val(&val), std::mem::size_of_val(&val_ptr), std::mem::size_of_val(&val_ref), std::mem::size_of_val(&val_ref_ptr));
    unsafe {
        println!("== Display val_ptr ==");
        let val_u32_ptr = val_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val) / std::mem::size_of::<u32>() {
            println!("val_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_u32_ptr.add(i), *val_u32_ptr.add(i));
        }
        println!("== Display val.as_ptr() ==");
        let val_as_ptr = val.as_ptr();
        for i in 0..val.len() {
            println!("val_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_as_ptr.add(i), *val_as_ptr.add(i));
        }

        println!("==Display reference==");
        let val_ref_ptr = val_ref_ptr as *const usize;
        println!("val_ref_ptr = {:?}, *val_ref_ptr = 0x{:016X}", val_ref_ptr, *val_ref_ptr);
    }

    println!("\n### vec: Move ###");
    println!("Before Move");
    let val: Vec<i32> = vec![0x10, 0x20, 0x30, 0x40, 0x50];
    let val_ptr: *const Vec<i32> = &val;
    println!("val = {:?}, val_ptr = {:?}, val.as_ptr() = {:?}", val, val_ptr, val.as_ptr());

    println!("After Move");
    let val_move = val;
    let val_move_ptr: *const _ = &val_move;
    // println!("val = {:?}, val_ptr = {:?}, val.as_ptr() = {:?}", val, val_ptr, val.as_ptr()); // Error: borrow of moved value
    println!("val_move = {:?}, val_move_ptr = {:?}, val_move.as_ptr() = {:?}", val_move, val_move_ptr, val_move.as_ptr());
    println!("sizeof(val_move) = {}, ", std::mem::size_of_val(&val_move));

    unsafe {
        println!("== Display val_ptr ==");
        let val_u32_ptr = val_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val_move) / std::mem::size_of::<u32>() {
            println!("val_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_u32_ptr.add(i), *val_u32_ptr.add(i));
        }

        // // === Error: borrow of moved value ===
        // println!("== Display val.as_ptr() ==");
        // let val_as_ptr = val.as_ptr();
        // for i in 0..val_move.len() {
        //     println!("val_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_as_ptr.add(i), *val_as_ptr.add(i));
        // }

        println!("== Display val_move_ptr ==");
        let val_move_u32_ptr = val_move_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val_move) / std::mem::size_of::<u32>() {
            println!("val_move_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_move_u32_ptr.add(i), *val_move_u32_ptr.add(i));
        }
        println!("== Display val_move.as_ptr() ==");
        let val_move_as_ptr = val_move.as_ptr();
        for i in 0..val_move.len() {
            println!("val_move_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_move_as_ptr.add(i), *val_move_as_ptr.add(i));
        }
    }


    println!("\n####################\n# &str\n####################");
    println!("\n### &str: Memory Allocation ###");
    let val: &str = "123";
    let val_ptr: *const &str = &val;
    let val_ref: &&str = &val;
    let val_ref_ptr: *const &&str = &val_ref;
    let val_as_ptr: *const u8 = val.as_ptr();
    println!("val = {:?}, val_ptr = {:?}, val_ref = {:?}, val_ref_ptr = {:?}, val_as_ptr = {:?}", val, val_ptr, val_ref, val_ref_ptr, val_as_ptr);
    println!("sizeof(val) = {}, sizeof(val_ptr) = {}, sizeof(val_ref) = {}, sizeof(val_ref_ptr) = {}",
        std::mem::size_of_val(&val), std::mem::size_of_val(&val_ptr), std::mem::size_of_val(&val_ref), std::mem::size_of_val(&val_ref_ptr));
    unsafe {
        println!("== Display val_ptr ==");
        let val_u32_ptr = val_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val) / std::mem::size_of::<u32>() {
            println!("val_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_u32_ptr.add(i), *val_u32_ptr.add(i));
        }
        println!("== Display val.as_ptr() ==");
        let val_as_ptr = val.as_ptr();
        for i in 0..val.len() {
            println!("val_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_as_ptr.add(i), *val_as_ptr.add(i));
        }

        println!("==Display reference==");
        let val_ref_ptr = val_ref_ptr as *const usize;
        println!("val_ref_ptr = {:?}, *val_ref_ptr = 0x{:016X}", val_ref_ptr, *val_ref_ptr);
    }

    println!("\n### &str: Move ###");
    println!("Before Move");
    let val: &str = "123";
    let val_ptr: *const &str = &val;
    println!("val = {:?}, val_ptr = {:?}, val.as_ptr() = {:?}", val, val_ptr, val.as_ptr());

    println!("After Move");
    let val_move = val;
    let val_move_ptr: *const _ = &val_move;
    // println!("val = {:?}, val_ptr = {:?}, val.as_ptr() = {:?}", val, val_ptr, val.as_ptr()); // Error: borrow of moved value
    println!("val_move = {:?}, val_move_ptr = {:?}, val_move.as_ptr() = {:?}", val_move, val_move_ptr, val_move.as_ptr());
    println!("sizeof(val_move) = {}, ", std::mem::size_of_val(&val_move));

    unsafe {
        println!("== Display val_ptr ==");
        let val_u32_ptr = val_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val_move) / std::mem::size_of::<u32>() {
            println!("val_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_u32_ptr.add(i), *val_u32_ptr.add(i));
        }

        println!("== Display val.as_ptr() ==");
        let val_as_ptr = val.as_ptr();
        for i in 0..val_move.len() {
            println!("val_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_as_ptr.add(i), *val_as_ptr.add(i));
        }

        println!("== Display val_move_ptr ==");
        let val_move_u32_ptr = val_move_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val_move) / std::mem::size_of::<u32>() {
            println!("val_move_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_move_u32_ptr.add(i), *val_move_u32_ptr.add(i));
        }
        println!("== Display val_move.as_ptr() ==");
        let val_move_as_ptr = val_move.as_ptr();
        for i in 0..val_move.len() {
            println!("val_move_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_move_as_ptr.add(i), *val_move_as_ptr.add(i));
        }
    }



    println!("\n####################\n# String\n####################");
    println!("\n### String: Memory Allocation ###");
    let val: String = "123".to_string();
    let val_ptr: *const String = &val;
    let val_ref: &String = &val;
    let val_ref_ptr: *const &String = &val_ref;
    let val_as_ptr: *const u8 = val.as_ptr();
    println!("val = {:?}, val_ptr = {:?}, val_ref = {:?}, val_ref_ptr = {:?}, val_as_ptr = {:?}", val, val_ptr, val_ref, val_ref_ptr, val_as_ptr);
    println!("sizeof(val) = {}, sizeof(val_ptr) = {}, sizeof(val_ref) = {}, sizeof(val_ref_ptr) = {}",
        std::mem::size_of_val(&val), std::mem::size_of_val(&val_ptr), std::mem::size_of_val(&val_ref), std::mem::size_of_val(&val_ref_ptr));
    unsafe {
        println!("== Display val_ptr ==");
        let val_u32_ptr = val_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val) / std::mem::size_of::<u32>() {
            println!("val_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_u32_ptr.add(i), *val_u32_ptr.add(i));
        }
        println!("== Display val.as_ptr() ==");
        let val_as_ptr = val.as_ptr();
        for i in 0..val.len() {
            println!("val_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_as_ptr.add(i), *val_as_ptr.add(i));
        }

        println!("==Display reference==");
        let val_ref_ptr = val_ref_ptr as *const usize;
        println!("val_ref_ptr = {:?}, *val_ref_ptr = 0x{:016X}", val_ref_ptr, *val_ref_ptr);
    }

    println!("\n### String: Move ###");
    println!("Before Move");
    let val: String = "123".to_string();
    let val_ptr: *const String = &val;
    println!("val = {:?}, val_ptr = {:?}, val.as_ptr() = {:?}", val, val_ptr, val.as_ptr());

    println!("After Move");
    let val_move = val;
    let val_move_ptr: *const _ = &val_move;
    // println!("val = {:?}, val_ptr = {:?}, val.as_ptr() = {:?}", val, val_ptr, val.as_ptr()); // Error: borrow of moved value
    println!("val_move = {:?}, val_move_ptr = {:?}, val_move.as_ptr() = {:?}", val_move, val_move_ptr, val_move.as_ptr());
    println!("sizeof(val_move) = {}, ", std::mem::size_of_val(&val_move));

    unsafe {
        println!("== Display val_ptr ==");
        let val_u32_ptr = val_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val_move) / std::mem::size_of::<u32>() {
            println!("val_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_u32_ptr.add(i), *val_u32_ptr.add(i));
        }

        // // === Error: borrow of moved value ===
        // println!("== Display val.as_ptr() ==");
        // let val_as_ptr = val.as_ptr();
        // for i in 0..val_move.len() {
        //     println!("val_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_as_ptr.add(i), *val_as_ptr.add(i));
        // }

        println!("== Display val_move_ptr ==");
        let val_move_u32_ptr = val_move_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val_move) / std::mem::size_of::<u32>() {
            println!("val_move_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_move_u32_ptr.add(i), *val_move_u32_ptr.add(i));
        }
        println!("== Display val_move.as_ptr() ==");
        let val_move_as_ptr = val_move.as_ptr();
        for i in 0..val_move.len() {
            println!("val_move_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_move_as_ptr.add(i), *val_move_as_ptr.add(i));
        }
    }




    println!("\n####################\n# struct\n####################");

    #[derive(Debug)]
    struct Person {
        id: i32,
        age: i32,
        name: String,
    }

    println!("\n### struct: Memory Allocation ###");
    let val = Person{ id: 1, age: 2, name: "ABC".to_string() };
    let val_ptr: *const _ = &val;
    let val_id_ptr: *const i32 = &val.id;
    let val_age_ptr: *const i32 = &val.age;
    let val_str_ptr: *const String = &val.name;
    println!("val_ptr = {:?}, val_id_ptr = {:?}, val_age_ptr = {:?}, val_str_ptr = {:?}, val.name.as_ptr() = {:?}", val_ptr, val_id_ptr, val_age_ptr, val_str_ptr, val.name.as_ptr());
    println!("sizeof(val) = {}", std::mem::size_of_val(&val));
    unsafe {
        println!("== Display val_ptr ==");
        let val_u32_ptr = val_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val) / std::mem::size_of::<u32>() {
            println!("val_u8_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_u32_ptr.add(i), *val_u32_ptr.add(i));
        }
    }

    println!("\n### struct: Move ###");
    println!("Before Move");
    let val = Person{ id: 1, age: 2, name: "ABC".to_string() };
    let val_ptr: *const Person = &val;
    println!("val_ptr = {:?}, val_id_ptr = {:?}, val_age_ptr = {:?}, val_str_ptr = {:?}, val.name.as_ptr() = {:?}", val_ptr, val_id_ptr, val_age_ptr, val_str_ptr, val.name.as_ptr());
    
    println!("After Move");
    let val_move = val;
    let val_move_ptr: *const _ = &val_move;
    let val_move_id_ptr: *const i32 = &val_move.id;
    let val_move_age_ptr: *const i32 = &val_move.age;
    let val_move_str_ptr: *const String = &val_move.name;
    // println!("val_ptr = {:?}, val_id_ptr = {:?}, val_age_ptr = {:?}, val_str_ptr = {:?}, val.name.as_ptr() = {:?}", val_ptr, val_id_ptr, val_age_ptr, val_str_ptr, val.name.as_ptr());
    println!("val_move_ptr = {:?}, val_move_id_ptr = {:?}, val_move_age_ptr = {:?}, val_move_str_ptr = {:?}, val_move.name.as_ptr() = {:?}", val_move_ptr, val_move_id_ptr, val_move_age_ptr, val_move_str_ptr, val_move.name.as_ptr());

    unsafe {
        println!("== Display val_ptr ==");
        let val_u32_ptr = val_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val_move) / std::mem::size_of::<u32>() {
            println!("val_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_u32_ptr.add(i), *val_u32_ptr.add(i));
        }

        println!("== Display val_move_ptr ==");
        let val_move_u32_ptr = val_move_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val_move) / std::mem::size_of::<u32>() {
            println!("val_move_u32_ptr.add({}) = {:?}, val = 0x{:08X}", i, val_move_u32_ptr.add(i), *val_move_u32_ptr.add(i));
        }
    }
}
