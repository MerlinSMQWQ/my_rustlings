fn main() {
    // 常量必须在定义的时候指定数据类型，最好定义为全大写，常量是无论如何也无法改变的
    const PI: f64 = 3.1415926;
    println!("PI = {}", PI);

    // rust常量甚至不允许被遮蔽，也无法重新定义
    // let PI: f64 = 1.23;

    // 除了使用const关键字，还可以使用static，static具有'static生命周期的可以是可变的变量，但是需要使用static mut关键字，访问可变静态变量需要 unsafe
    static NAME: &'static str = "Merlin";
    print!("My name is {}", NAME);
}