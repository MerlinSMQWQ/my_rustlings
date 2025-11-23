fn main() {
    // let 数组名: [数据类型; 数组长度] = [value1, value2,value3]
    let arr1: [i32; 4] = [1, 2, 3, 4];
    let mut names: [&str; 3] = ["Alice", "Bob", "David"];

    // 查看数组的长度
    println!("{}", arr1.len());

    // 数组名[下标]，访问数组元素
    println!("{}", arr1[1]);

    // 直接遍历会发生所有权转移，如果没有实现copy trait，遍历后改数组将无法继续使用
    for item in names {
        println!("I am {}", item);
    }

    // 使用iter()不会发生所有权转移，如果要读一个只读的数组，最好使用上iter()
    for item in names.iter() {
        println!("You are {}", item);
    }

    // 同样，&str实现了copy trait，即使是into_iter后也可以继续使用
    // for name in names.into_iter() {
    //     println!("点到了：{}", name);
    // }
    println!("{:?}", names);
    show_names(names);
    println!("{:?}", names);
    change_names(&mut names);
    println!("{:?}", names);
}

// 数组也是值传递
fn show_names(mut names: [&str; 3]) {
    let l = names.len();
    for i in 0..l {
        if i == 0 {
            names[i] = "Eva";
        }
    }
    println!("{:?}", names);
}

fn change_names(names: &mut [&str; 3]) {
    let l = names.len();
    for i in 0..l {
        if i == 0 {
            names[i] = "Eva";
        }
    }
}