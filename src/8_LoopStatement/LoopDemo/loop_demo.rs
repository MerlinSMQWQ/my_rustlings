fn main() {
    // loop和break组合，实现条件循环，如果没有break就是一个死循环
    let mut num = 1;
    loop {
        if num > 20 {
            break;
        }
        println!("num is {}", num);
        num = num * 3;
    }
}
