fn test_thread() {
    use std::io::Read;
    let handle0 = std::thread::spawn(|| {
        for i in 0..10 {
            println!("thread #0 {}", i);
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });

    let handle1 = std::thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 {}", i);
            std::thread::sleep(std::time::Duration::from_millis(800));
        }
    });

    // std::io::stdin().read(&mut [0]);
    println!("join handle0");
    handle0.join().unwrap();
    println!("join handle1");
    handle1.join().unwrap();
}

fn test_thread2() {
    fn foo(id: i32) {
        for i in 0..10 {
            println!("thread #{} {}", id, i);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    std::thread::spawn(|| {foo(10);}).join().unwrap();
    std::thread::spawn(|| {foo(20);}).join().unwrap();
    std::thread::spawn(|| {foo(30);}).join().unwrap();
}

fn test_async() {
    async fn foo(id: i32) {
        for i in 0..10 {
            println!("thread #{} {}", id, i);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    let task = async {
        foo(10).await;
        foo(20).await;
        foo(30).await;
    };
    println!("start");
    futures::executor::block_on(task);
    println!("end");
}

async fn sub_async() {
    async fn foo(id: i32) {
        for i in 0..10 {
            println!("thread #{} {}", id, i);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    foo(10).await;
    foo(20).await;
    foo(30).await;
}

fn test_move_1() {
    let pool = futures::executor::ThreadPool::new().unwrap();
    let task = async {
        for j in 1..6 {
            let mut id = j * 10;
            println!("after before = {}", id);
            pool.spawn_ok(async move {
                for i in 0..10 {
                    println!("thread #{}, {}", id, i);
                    id += 1;
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            });
            println!("after id = {}", id);
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    };
    futures::executor::block_on(task);
    use std::io::Read;
    let _ = std::io::stdin().read(&mut [0]);
}

#[tokio::main]
async fn main() {
    // test_thread();
    // test_thread2();
    // test_async();
    // futures::executor::block_on(sub_async());
    // sub_async().await;
    
    test_move_1();

}
