use std::{thread, time::Duration};

fn main() {
    // 使用std::thread::spawn() 创建一个线程
    // 主线程结束以后，子线程也会结束，即使任务没有完成
    // thread::sleep()会让线程睡眠一段时间
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("子线程{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("主线程{}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 为了防止主线程任务结束后所有子线程都结束，我们需要使用Join方法阻塞主线程，直到子线程结束，主线程才结束并退出
    handler.join().unwrap();
}