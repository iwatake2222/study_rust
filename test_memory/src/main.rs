fn main() {
    println!("\n=== i32: Memory allocation ===");
    let mut val0: i32 = 12345678;
    let val0_ptr: *mut i32 = &mut val0;
    println!("val0 = {}, val0_ptr = {:?}", val0, val0_ptr);
    println!("sizeof(val0) = {}, sizeof(val0_ptr) = {}", std::mem::size_of_val(&val0), std::mem::size_of_val(&val0_ptr));

    println!("\n=== Pointer access ===");
    unsafe {
        *val0_ptr += 1;
        println!("val0 + 1 = {}", val0);
        println!("val0_ptr.add(1) = {:?}", val0_ptr.add(1));
        // println!("val0_ptr + 1 = {:?}", val0_ptr + (1 as *mut i32));
    }

    println!("\n=== Pointer cast ===");
    // unsafe {
    //     let null_ptr = 0x0000_0000_0000_0000 as *mut u32;
    //     *null_ptr = 0xdeadbeef;
    // }
    let mut val0: i32 = 0x12345678;
    let val0_ptr: *mut i32 = &mut val0;
    let val0_u8_ptr: *mut u8 = val0_ptr as *mut u8;
    println!("val0 = 0x{:08X}, val0_ptr = {:?}, val0_u8_ptr = {:?}", val0, val0_ptr, val0_u8_ptr);
    unsafe {
        for i in 0..4 {
            println!("val0_u8_ptr.add({}) = {:?}, val = 0x{:02X}", i, val0_u8_ptr.add(i), *val0_u8_ptr.add(i));
        }
    }

    println!("\n=== i32: Reference ===");
    let val0: i32 = 12345678;
    let val0_ptr: *const i32 = &val0;
    let val0_ref= &val0;
    let val0_ref_ptr: *const &i32 = &val0_ref;
    
    println!("val0 = {:?}, val0_ptr = {:?}, val0_ref = {:?}, val0_ref_ptr = {:?}", val0, val0_ptr, val0_ref, val0_ref_ptr);
    println!("sizeof(val0) = {}, sizeof(val0_ptr) = {}, sizeof(val0_ref) = {}, sizeof(val0_ref_ptr) = {}",
        std::mem::size_of_val(&val0), std::mem::size_of_val(&val0_ptr), std::mem::size_of_val(&val0_ref), std::mem::size_of_val(&val0_ref_ptr));

    unsafe {
        println!("*val0_ptr = {:?}", *val0_ptr);
        println!("val0_ref() = {}, *val0_ref() = {}", val0_ref, *val0_ref); // automatically derefeered
        println!("val0_ref_ptr() = {:?}, *val0_ref_ptr() = {:?}", val0_ref_ptr, *val0_ref_ptr); // automatically derefeered
        let val0_ref_ptr = val0_ref_ptr as *const usize;
        println!("val0_ref_ptr() = {:?}, *val0_ref_ptr() = 0x{:016X}", val0_ref_ptr, *val0_ref_ptr);
    }


    println!("\n=== i32: Move ===");
    println!("val0 = {}, val0_ptr = {:?}", val0, val0_ptr);

    let val1 = val0;
    let val1_ptr: *const _ = &val1;
    println!("val0 = {}, val0_ptr = {:?}", val0, val0_ptr);
    println!("val1 = {}, val1_ptr = {:?}", val1, val1_ptr);


    println!("\n=== vec: Memory allocation ===");
    let val0: Vec<i32> = vec![0x10, 0x20, 0x30];
    let val0_ptr: *const _ = &val0;
    println!("val0 = {:?}, val0_ptr = {:?}, val0.as_ptr() = {:?}", val0, val0_ptr, val0.as_ptr());
    println!("sizeof(val0) = {}", std::mem::size_of_val(&val0));
    unsafe {
        println!("== Display val0_ptr ==");
        let val0_u32_ptr = val0_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val0) / std::mem::size_of::<u32>() {
            println!("val0_u8_ptr.add({}) = {:?}, val = 0x{:08X}", i, val0_u32_ptr.add(i), *val0_u32_ptr.add(i));
        }
        println!("== Display val0.as_ptr() ==");
        let val0_as_ptr = val0.as_ptr();
        for i in 0..3 {
            println!("val0_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val0_as_ptr.add(i), *val0_as_ptr.add(i));
        }
    }

    println!("\n=== vec: Move ===");
    println!("val0 = {:?}, val0_ptr = {:?}, val0.as_ptr() = {:?}", val0, val0_ptr, val0.as_ptr());
    let val1 = val0;
    let val1_ptr: *const _ = &val1;
    // println!("val0 = {:?}, val0_ptr = {:?}, val0.as_ptr() = {:?}", val0, val0_ptr, val0.as_ptr());
    println!("val1 = {:?}, val1_ptr = {:?}, val1.as_ptr() = {:?}", val1, val1_ptr, val1.as_ptr());
    println!("sizeof(val1) = {}, ", std::mem::size_of_val(&val1));

    println!("\n=== vec: Reference ===");
    let val0: Vec<i32> = vec![0x10, 0x20, 0x30];
    let val1 = &val0;
    let val1_ptr: *const _ = &val1;
    println!("val0 = {:?}, val0_ptr = {:?}, val0.as_ptr() = {:?}", val0, val0_ptr, val0.as_ptr());
    println!("val1 = {:?}, val1_ptr = {:?}, val1.as_ptr() = {:?}", val1, val1_ptr, val1.as_ptr());
    println!("sizeof(val1) = {}, ", std::mem::size_of_val(&val1));


    println!("\n=== &str: Memory allocation ===");
    let val0: &str = "123";
    let val0_ptr: *const _ = &val0;
    println!("val0 = {:?}, val0_ptr = {:?}, val0.as_ptr() = {:?}", val0, val0_ptr, val0.as_ptr());
    println!("sizeof(val0) = {}", std::mem::size_of_val(&val0));
    unsafe {
        println!("== Display val0_ptr ==");
        let val0_u32_ptr = val0_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val0) / std::mem::size_of::<u32>() {
            println!("val0_u8_ptr.add({}) = {:?}, val = 0x{:08X}", i, val0_u32_ptr.add(i), *val0_u32_ptr.add(i));
        }
        println!("== Display val0.as_ptr() ==");
        let val0_as_ptr = val0.as_ptr();
        for i in 0..3 {
            println!("val0_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val0_as_ptr.add(i), *val0_as_ptr.add(i));
        }
    }

    println!("\n=== &str: Move ===");
    println!("val0 = {:?}, val0_ptr = {:?}, val0.as_ptr() = {:?}", val0, val0_ptr, val0.as_ptr());
    let val1 = val0;
    let val1_ptr: *const _ = &val1;
    println!("val0 = {:?}, val0_ptr = {:?}, val0.as_ptr() = {:?}", val0, val0_ptr, val0.as_ptr());
    println!("val1 = {:?}, val1_ptr = {:?}, val1.as_ptr() = {:?}", val1, val1_ptr, val1.as_ptr());

    println!("\n=== String: Memory allocation ===");
    let val0: String = "123".to_string();
    let val0_ptr: *const _ = &val0;
    println!("val0 = {:?}, val0_ptr = {:?}, val0.as_ptr() = {:?}", val0, val0_ptr, val0.as_ptr());
    println!("sizeof(val0) = {}", std::mem::size_of_val(&val0));
    unsafe {
        println!("== Display val0_ptr ==");
        let val0_u32_ptr = val0_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val0) / std::mem::size_of::<u32>() {
            println!("val0_u8_ptr.add({}) = {:?}, val = 0x{:08X}", i, val0_u32_ptr.add(i), *val0_u32_ptr.add(i));
        }
        println!("== Display val0.as_ptr() ==");
        let val0_as_ptr = val0.as_ptr();
        for i in 0..3 {
            println!("val0_as_ptr.add({}) = {:?}, val = 0x{:08X}", i, val0_as_ptr.add(i), *val0_as_ptr.add(i));
        }
    }

    println!("\n=== String: Move ===");
    println!("val0 = {:?}, val0_ptr = {:?}, val0.as_ptr() = {:?}", val0, val0_ptr, val0.as_ptr());
    let val1 = val0;
    let val1_ptr: *const _ = &val1;
    // println!("val0 = {:?}, val0_ptr = {:?}, val0.as_ptr() = {:?}", val0, val0_ptr, val0.as_ptr());
    println!("val1 = {:?}, val1_ptr = {:?}, val1.as_ptr() = {:?}", val1, val1_ptr, val1.as_ptr());

    println!("\n=== struct: Memory allocation ===");
    #[derive(Debug)]
    struct Person {
        id: i32,
        age: i32,
        name: String,
    }
    let val0 = Person{ id: 1, age: 2, name: "ABC".to_string() };
    let val0_ptr: *const _ = &val0;
    let val0_id_ptr: *const i32 = &val0.id;
    let val0_age_ptr: *const i32 = &val0.age;
    let val0_str_ptr: *const String = &val0.name;
    println!("val0_ptr = {:?}, val0_id_ptr = {:?}, val0_age_ptr = {:?}, val0_str_ptr = {:?}, val0.name.as_ptr() = {:?}", val0_ptr, val0_id_ptr, val0_age_ptr, val0_str_ptr, val0.name.as_ptr());
    println!("sizeof(val0) = {}", std::mem::size_of_val(&val0));
    unsafe {
        println!("== Display val0_ptr ==");
        let val0_u32_ptr = val0_ptr as *const u32;
        for i in 0..std::mem::size_of_val(&val0) / std::mem::size_of::<u32>() {
            println!("val0_u8_ptr.add({}) = {:?}, val = 0x{:08X}", i, val0_u32_ptr.add(i), *val0_u32_ptr.add(i));
        }
    }


    println!("\n=== struct: Move ===");
    println!("val0_ptr = {:?}, val0_id_ptr = {:?}, val0_age_ptr = {:?}, val0_str_ptr = {:?}, val0.name.as_ptr() = {:?}", val0_ptr, val0_id_ptr, val0_age_ptr, val0_str_ptr, val0.name.as_ptr());
    let val1 = val0;
    let val1_ptr: *const _ = &val1;
    let val1_id_ptr: *const i32 = &val1.id;
    let val1_age_ptr: *const i32 = &val1.age;
    let val1_str_ptr: *const String = &val1.name;
    // println!("val0_ptr = {:?}, val0_id_ptr = {:?}, val0_age_ptr = {:?}, val0_str_ptr = {:?}, val0.name.as_ptr() = {:?}", val0_ptr, val0_id_ptr, val0_age_ptr, val0_str_ptr, val0.name.as_ptr());
    println!("val1_ptr = {:?}, val1_id_ptr = {:?}, val1_age_ptr = {:?}, val1_str_ptr = {:?}, val1.name.as_ptr() = {:?}", val1_ptr, val1_id_ptr, val1_age_ptr, val1_str_ptr, val1.name.as_ptr());


}
