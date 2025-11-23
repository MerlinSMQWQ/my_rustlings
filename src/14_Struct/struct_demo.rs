// rust中有三种结构体，分别是元组结构体（tuple struct）、C语言风格的结构体（C struct）、单元结构体（unit struct）
// 例如这是元组结构体
// struct Point(i32, i32);
// 例如这是一个单元结构体
// struct EmptyStruct;

// 自动生成Debug trait的实现代码
#[derive(Debug)]
struct Student {
    name: String,
    gender: String,
    id: i32
}
    

fn main() {
    let mut s = Student {
        name: String::from("Alice"),
        gender: String::from("Girl"),
        id: 1
    };

    show_struct(&s);
    println!("{:#?}", s);
    // 调用普通方法
    println!("s.id = {}", s.get_id());
    // 调用静态方法
    println!("it is a {}", Student::who());
    // 调用可变方法
    s.set_name(String::from("Alice"));
    // 调用消费方法
    println!("it is a {}", s.get_name());
    // println!("{:?}", s); 无法再次使用s实例
}

// 依然是引用传参，不会讲s本身的所有权转移出去
fn show_struct(s: &Student) {
    println!("{}", s.name);
    println!("{}", s.gender);
    println!("{}", s.id);
}

impl Student {
    // 一般方法，需要传入&self，表示实例本身的引用
    fn get_id(&self) -> i32 {
        return self.id;
    }

    // 可变方法
    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    // 静态方法
    fn who() -> String {
        return String::from("student");
    }

    // 消费方法，实例在调用后无法再使用
    fn get_name(self) -> String {
        // 不能写成self.&name，因为.的优先级高于&，所以我们写成&self.name
        return self.name;
    }
}