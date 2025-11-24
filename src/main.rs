use std::fs::File;

fn main() {
    let v = vec!["Alice", "Bob", "Eva"];
    // println!("{}", v[5]);    这里是数组的越界访问，会抛出一个错误
    println!("{}", v[2]);

    // unwrap是 Result<T, E>的方法，在实例上调用此方法时，如果是 Ok 枚举值，就会返回 Ok 中的对象，如果是 Err 枚举值，在运行时会 panic，报错信息是 format!(“{}”, error)。其缺点是，如果在不同地方都使用 unwrap，运行时出现 panic 的时候
    let result1 = is_even(6).unwrap();
    let result2 = is_even(11).unwrap();
    println!("{:?}", result1);
    println!("{:?}", result2);


    // 我们可以使用expect()自定义报错信息，因此出现panic时比较容易定位
    let f = File::open("a.txt").expect("找不到文件");
    println!("{:?}", f);

    // panic!() 程序理解退出并抛出退出原因，也就是不可恢复的错误（异常）
    // 一般情况下，当遇到不可恢复错误时，程序会自动调用 panic!()
    panic!("出错了！");
    // 所以之后的代码不再会编译执行
    // println!("Hello Rust!");
}

// 我们使用Result枚举，Ok是正常时返回的值，而Err是错误的时候返回的值，我们可以自己定义返回什么，以及错误条件
fn is_even(num: i32) -> Result<bool, String> {
    return if num % 2 == 0 {
        Ok(true)
    } else {
        Err("输入值，不是偶数".to_string())
    }
}