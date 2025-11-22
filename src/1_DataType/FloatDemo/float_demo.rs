fn main() {
    // rust强调类型必须匹配，比如下面这一行就会报错，因为99不是f64类型，rust不会自动进行类型转换
    // let num: f64 = 99;
    let num: f64 = 99.0;
    let num2: f32 = 8.88;
    let num3: f64 = 1.23;

    println!("num = {}", num);
    println!("num2 = {}", num2);
    println!("num3 = {}", num3);

    // 同样的，浮点数也是可以用下划线进行分隔的
    let num4 = 1.234_567;
    println!("num4 = {}", num4);
}