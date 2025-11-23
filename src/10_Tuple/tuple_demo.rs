fn main() {
    let t = ("Hello", "world", "Hello Rust!");
    // 元组变量.索引下标
    println!("{}", t.0);
    println!("{}", t.1);
    println!("{}", t.2);
    show_tuple(t);

    // 元组的解包赋值
    let (s1, s2, s3) = t;
    println!("{}\t{}\t{}", s1, s2, s3);
}

fn show_tuple(t: (&str, &str, &str)) {
    /*
        {}是默认格式化，实现Display trait
        {:?}是调试格式化，实现Debug trait
        {:#?}是美化调试格式化，输出更加易读的多行格式
        {:p} - 输出指针地址
        {:x} - 十六进制小写输出
        {:X} - 十六进制大写输出
        {:o} - 八进制输出
        {:b} - 二进制输出
        {:e} - 科学计数法输出
     */
    println!("{:?}", t);
}