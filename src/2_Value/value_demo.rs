fn main() {
    // rust中定义变量可以指定类型也可以不指定，不指定rust会自行进行判断
    /*
        变量命名的规范：
            可以包含数字、字母、下划线
            必须以字母或者下划线开头，不能以数字开头
            变量名大小写敏感
    */
    let name = "Merlin";
    let name2: &str = "QWQ";


    println!("I am {}{}", name,name2);

    // rust变量默认是不可变的，我们如果要修改不可变变量就会报错，例如
    // name2 = "qwq"
    // 想要使变量可变需要使用mut关键字(mutable的缩写，必须和let一起使用)，设置变量为可变变量，变量一旦被设置为可变就无法设置会不可变了，除非使用覆盖
    let mut num = 1;
    println!("num = {}", num);
    num = 2;
    println!("num = {}", num);
    // 如果重新使用let，则会发生覆盖
    let name = "merlin";
    println!("I am {}{}", name, name2);
}