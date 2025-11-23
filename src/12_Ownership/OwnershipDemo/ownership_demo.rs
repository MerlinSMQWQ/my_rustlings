use std::vec;

fn main() {
    // 大小确定的数据放在栈上，而大小不确定的数据放在堆上，堆不受操作系统管理，需要用户自己管理，有较大的内存溢出风险
    
    let name: String = String::from("Merlin");
    let name2 = name;
    // 我们无法再访问name了，因为所有权发生了转移
    // println!("{}", name);
    println!("{}", name2);

    // 这里就不会发生所有权转移，因为i32类型的数据放在栈上，大小确定，由操作系统管理，实现了copy trait
    let num = 1;
    let num2 = num;
    println!("{}", num);
    println!("{}", num2);

    // 容器Vec因为长度可变，所以放在堆上，实现的是move trait
    let v1 = vec!["hello", "world", "rust"];
    let v2 = v1;
    // println!("{:?}", v1);  这里就无法使用v1因为所有权转移
    println!("{:?}", v2);

    let v = vec!["Alice", "Bob", "Cline"];
    // 函数传递的时候也发生的所有权转移
    show_vec(v);
    // println!("{:?}", v); 因此这里无法打印出来

    let v = vec!["Alice", "Bob", "Eva"];
    // 所以我们要传递引用
    show_vec2(&v);
    println!("v: {:?}", v);
}

fn show_vec(v: Vec<&str>) {
    println!("v: {:?}", v);
}

fn show_vec2(v: &Vec<&str>) {
    println!("v: {:?}", v);
}