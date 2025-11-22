// rust提供了一个非常强大的控制流运算符match
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}

#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {    // 如果这里是多行的代码块，那就需要使用花括号括起来，有点像匿名函数
            println!("this is a penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}!", state);
            25
        },
    }
}

// 对于Option<T>，我们也可以使用match
fn push_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));
    let nickel = Coin::Nickel;
    println!("{}", value_in_cents(nickel));
    let dime = Coin::Dime;
    println!("{}", value_in_cents(dime));
    let quarter = Coin::Quarter(USState::Alabama);
    println!("{}", value_in_cents(quarter));
    let quarter = Coin::Quarter(USState::Alaska);
    println!("{}", value_in_cents(quarter));


    let five = push_one(Some(5));
    let none = push_one(None);
    println!("{:?}", five);
    println!("{:?}", none);

    // match必须包含所有情况，为了方便，match也可以有默认处理
    let x = 0u8;
    match x {
        1 => println!("1"),
        _ => println!("{}", x), // 默认情况处理，放在最后面
    }

    // 为了更简单地进行匹配，rust还有if let语句，如果只要匹配一种情况，if let可以用更少的代码，达到同样的目的(可以讲if let看做match的一个语法糖，match更通用)
    if let 1 = x {  // 这里和正常的逻辑有点不一样，是反着的
        println!("1");
    } else {
        println!("{x}");
    }
}