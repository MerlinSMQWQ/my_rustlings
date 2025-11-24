use std::collections::HashSet;

fn main() {
    /*
        HashSet是相同数据类型的元素的集合，没有重复的值，如果插入重复的值，就会失败
        创建方法：
            let mut 变量名 = HashSet::new();
     */
    let mut student_set = HashSet::new();
    student_set.insert("Alex");
    student_set.insert("Bob");
    student_set.insert("Eva");

    println!("{:?}", student_set);

    // 插入已经存在的值
    let result = student_set.insert("Eva");
    println!("is inserted: {}", result);
    println!("{:?}", student_set);

    // 查看集合中元素的个数
    println!("length of student_set: {}", student_set.len());

    // HashSet没有顺序
    for item in student_set.iter() {
        println!("student_set: {:?}", item)
    }

    // 获取元素
    match student_set.get("Eva") {
        None => { println!("找不到") }
        Some(data) => {println!("找到了：{}", data)}
    }

    // 查找元素
    if student_set.contains("Alex") {
        println!("找到Alex了!");
    }

    // 删除元素
    let result = student_set.remove("Bob");
    println!("is removed: {}", result);
    println!("{:?}", student_set);

    let result = student_set.remove("Cline");
    println!("is removed: {}", result);
    println!("{:?}", student_set);
}