fn main() {
    // 1..5左开右闭
    for num in 1..5 {
        println!("{}", num);
    }

    println!("<=================================>");

    // 1..=5左右都是闭
    for num in 1..=5 {
        println!("{}", num);
    }

    println!("<=================================>");

    let student_list = vec![
        "Alice",
        "Bob",
        "Cline"
    ];
    // iter()每次迭代都是借用集合中的一个元素，元素本身不会改变，循环之后依旧可以使用
    for name in student_list.iter() {
        match name {
            /*
                在 Rust 中，& 是因为 name 是一个引用，而模式匹配需要解引用。
                student_list.iter() 产生的是元素的引用（即 &str 类型）所以 name 的类型是 &&str在 match 中使用 &"Alice" 是为了匹配这个双重引用：
                    name 是 &&str"Alice" 是 &str
                    &"Alice" 就变成了 
                    &&str与 name 类型一致
             */
            &"Alice" => println!("Hello, Alice"),
            _ => println!("Hello, classmate!")
        }
    }

    println!("<=================================>");

    // into_iter()是会消耗集合元素的，每次迭代，都会消耗，消耗完了则无法使用
    for name in student_list.into_iter() {
        match name {
            "Bob" => println!("Hello, Bob"),
            _ => println!("Hello, classmate!")
        }
    }

    println!("<=================================>");
}
