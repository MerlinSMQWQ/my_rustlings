fn main() {
    /*
    fn 函数名([参数: 参数类型]) -> 返回值 {
        函数体
    }

    无明确返回值得时候，就会返回一个单元类型()
     */
    out_str(get_str1());
    out_str(get_str2());
    
    let num = 12;
    calculator(num);
    let mut num = 12;
    neo_calculator(&mut num);
    println!("outside: num = {}", num);

    let words: String = String::from("hello");
    out_str(words);
    // println!("{}", words);   无法执行这一句，因为words的所有权给了out_str()函数，String不是基本数据类型，需要考虑借用的问题（实现的是move的trait）,而基本数据类型实现的是copy的trait不需要考虑借用
}

// 复合类型传参
fn out_str(str: String) {
    println!("{}", str);
}

fn get_str1() -> String {
    return String::from("Hello, rust!")
}

fn get_str2() -> String {
    // 不带冒号，返回一个值，是一个表达式而不是一个语句，不需要用return显式地return（官方推荐）
    String::from("Hello, world!")
}

// 值传递，传入一个参数，且函数内这个参数是可变的，但是外部不会发生任何变化
fn calculator(mut num: i32) {
    num *= 2;
    println!("{}", num);
}

// 引用传递，传递的是变量的内存地址，是可以修改变量本身的内容的
fn neo_calculator(num: &mut i32) {
    // 星号是用解引用的
    *num *= 2;
    println!("inside: num = {}", num);
}
