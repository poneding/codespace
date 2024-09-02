use std::thread;

fn main() {
    start_one_thread();
    start_one_thread_result();
    start_two_threads();

    start_n_threads();
}

pub fn start_one_thread() {
    let handle = thread::spawn(|| {
        println!("Hello from a thread!");
    });

    handle.join().unwrap(); // 等待线程执行完成
}

pub fn start_one_thread_result() {
    let handle = thread::spawn(|| {
        println!("Hell from a thread!");
        200
    });

    match handle.join() {
        Ok(v) => println!("thread result: {}", v),
        Err(e) => println!("error: {:?}", e),
    }
}

pub fn start_two_threads() {
    let handle1 = thread::spawn(|| {
        println!("Hello from a thread1!");
    });

    let handle2 = thread::spawn(|| {
        println!("Hello from a thread2!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

pub fn start_n_threads() {
    const N: isize = 10;

    let handles = (0..N).map(|i| {
        thread::spawn(move || {
            println!("Hello from a thread{}!", i);
        })
    });

    handles.for_each(|handle|{
        handle.join().unwrap()
    })

    // for handle in handles {
    //     handle.join().unwrap();
    // }
}
