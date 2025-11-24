use std::fmt::Display;

fn main() {
    /*
        rust中的泛型有：泛型集合、泛型结构体、泛型函数、泛型枚举、trait
     */

    // 泛型集合
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push("4".parse().unwrap());   // 泛型会对数据类型进行约束

    let test1: Data<i32> = Data{data: 1};
    println!("{:?}", test1);
    println!("{}", test1.data);

    let test2: Data<f32> = Data{data: 12.3};
    println!("{:?}", test2);
    println!("{}", test2.data);

    let book = Book {
        name: String::from("Book1"),
        id: 1,
        author: String::from("Author1")
    };
    book.show();

    let book2 = Book {
        name: String::from("Book2"),
        id: 2,
        author: String::from("Author2")
    };
    // 调用泛型函数
    show2(book2);

    // 使用泛型枚举
    let some_number: MyOption<i32> = MyOption::Some(42);
    let none_value: MyOption<String> = MyOption::None;
    
    println!("{:?}", some_number);
    println!("{:?}", none_value);
    
    let success: MyResult<i32, String> = MyResult::Ok(100);
    let error: MyResult<i32, String> = MyResult::Err(String::from("Something went wrong"));
    
    println!("{:?}", success);
    println!("{:?}", error);
}

// 泛型结构体
#[derive(Debug)]
struct Data<T> {
    data: T
}

// trait
#[derive(Debug)]
struct Book {
    name: String,
    id: u32,
    author: String
}

trait ShowBook {
    fn show(&self);
}

impl ShowBook for Book{
    fn show(&self) {
        println!("Book name: {}, Book id: {}, Book author {}", self.name, self.id, self.author);
    }
}

// 泛型函数，要求类型必须实现Display trait
fn show2<T: Display>(t: T) {
    println!("{}", t);
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Book name: {}, Book id: {}, Book author {}", self.name, self.id, self.author);
        return Result::Ok(());
    }
}


// 泛型枚举定义
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}