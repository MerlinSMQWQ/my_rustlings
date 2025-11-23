/*
    enum 枚举名称 {
        variant1,
        variant2,
        variant3,
        ...
    };
*/

#[derive(Debug)]
enum Country {
    China,
    America,
    Britain,
    France,
    Russia
}
/*
Option经常用于函数返回值，可以有返回值，也可以无返回值
enum Option {
    Some(T),    // 用于返回一个值
    None    // 用于返回null，用None来替代
}
 */

fn main() {
    let country1 = Country::America;
    println!("country1: {:?}", country1);

    let num = 99;
    let result = judge(num);
    println!("{:?}", result);

    match_country(country1);
}

fn judge(num: i32) -> Option<bool>{
    if num > 100 {
        return Some(true);
    } else {
        return None;
    }
}

// 枚举配合match
fn match_country(country: Country) {
    match country {
        Country::China => println!("I love China!"),
        Country::America => println!("this is America"),
        _ => println!("other country") 
    }
}