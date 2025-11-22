fn main () {
    // rust会自动判断数据类型（这种判断是有限度的）
    // rust分为有符号整形和无符号整形，有符号整形以i开头，无符号整形以u开头，rust还可以按照存储空间选择不同长度的整形
    // 按照1字节、2字节、4字节、8字节和16字节划分长度（字节8位）
    // 我们可以通过使用std::u128::MAX或者std::i128::MIN来输出最大值和最小值
    let num1 = 100;
    let num2: u32 = 200;
    let num3: i32 = 300;
    let num4: isize = 400;
    let num5: usize = 500;

    println!("num1 = {}", num1);
    println!("num2 = {}", num2);
    println!("num3 = {}", num3);
    println!("num4 = {}", num4);
    println!("num5 = {}", num5);

    println!("std::u128::MAX = {}", std::u128::MAX);
    println!("std::i128::MIN = {}", std::i128::MIN);
}