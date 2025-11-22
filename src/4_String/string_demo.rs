fn main() {
    // &str是字符串字面量（字符串切片），字符串字面量从创建时就存在，一直到程序结束
    let name = "Merlin";
    println!("I am {}", name);
    // 字符串对象
    // String::new()创建一个新的空字符串，是静态的
    // String::from()从具体的字符串字面量创建字符串对象
    let s1 = String::new();
    println!("s1 = {}, s1.len = {}", s1, s1.len());

    let s2 = String::from("hello");
    println!("s2 = {}, s2.len = {}", s2, s2.len());

    let mut s3 = String::new();
    // push_str是添加一个字符串字面量
    s3.push_str(name);
    println!("s3 = {}, s3.len = {}", s3, s3.len());
    // push是添加一个字符
    s3.push('!');
    println!("s3 = {}, s3.len = {}", s3, s3.len());

    // replace()方法可以将原字符串中指定的切片替换成指定内容，并输出新的字符串
    let s4 = String::from("Hello World!");
    let result = s4.replace("World", "Rust");
    println!("{}", result);

    // to_string方法可以将字符串字面量转成字符串对象
    let s5 = "Hello Rust!".to_string();
    print!("{}", s5);

    // as_str方法将字符串对象转成字符串字面量
    let s6 = String::from("Hello Rust!");
    println!("{}", s6.as_str());

    // trim方法可以用于去除字符串头尾的空白符（制表符\t，回车\r，换行符\n等）
    let s7 = "\tHello World\t\r\n";
    println!("s7.len() = {}", s7.len());
    println!("s7.len() = {}", s7.trim().len());
    println!("{}", s7.trim());

    // split方法可以根据指定字符或字符串分隔字符串，返回一个迭代器
    let s8 = "Hello, World, Rust";
    for item in s8.split(", ") {
        println!("{}", item);
    }

    // chars将字符串分割为字符，返回一个迭代器
    let s9 = "Hello World!";
    for ch in s9.chars() {
        println!("{}", ch);
    }

    // 字符串拼接，一个字符串对象加上至少一个字符串切片(字符串字面量)
    let s10 = s9.to_string() + s9 + s9;
    println!("{}", s10);
}