use std::collections::HashMap;

fn main() {
    /*
        HashMap 键值对集合，键不能重复，值可以重复，需要显式导入std::collections

        创建HashMap：
        let mut 变量名称 = HashMap::new();

        使用insert方法插入键值对
     */

    let mut student: HashMap<i32, &str> = HashMap::new();
    student.insert(1,"Alice");
    student.insert(2, "Bob");
    student.insert(3, "Eva");

    println!("{:?}", student);
    println!("{}", student.len());

    // 按照key查找value
    let i: i32 = 2;
    match student.get(&i) {
        Some(v) => {
            println!("HashMap v: {}", v);
        }
        None => {
            println!("找不到");
        }
    }

    // 查找key是否被包含
    if student.contains_key(&i) {
        println!("找到了");
    };

    // 按照键删除键值对
    let i: i32 = 1;
    // remove返回的是有个Option枚举，并且移除指定key的键值对
    let x = student.remove(&i);
    println!("{:?}", x);

    println!("{:?}", student);
}