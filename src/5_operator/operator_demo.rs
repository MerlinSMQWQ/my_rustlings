// rust不支持自增和自减运算符（++和--）
// rust不支持自增和自减运算符（++和--）
fn main() {
    // ==================== 算术运算符 ====================
    let a = 10;
    let b = 3;
    
    println!("=== 算术运算符 ===");
    println!("{} + {} = {}", a, b, a + b);  // 加法
    println!("{} - {} = {}", a, b, a - b);  // 减法
    println!("{} * {} = {}", a, b, a * b);  // 乘法
    println!("{} / {} = {}", a, b, a / b);  // 除法
    println!("{} % {} = {}", a, b, a % b);  // 取模（求余数）
    
    // 注意：Rust 不支持 ++ 和 -- 运算符
    // let mut x = 5;
    // x++;  // 错误！Rust 不支持这种写法
    // 正确的写法：
    let mut x = 5;
    x += 1;  // 等价于 x = x + 1
    println!("x after increment: {}", x);
    
    // ==================== 比较运算符 ====================
    let c = 5;
    let d = 10;
    
    println!("\n=== 比较运算符 ===");
    println!("{} == {} : {}", c, d, c == d);  // 等于
    println!("{} != {} : {}", c, d, c != d);  // 不等于
    println!("{} < {} : {}", c, d, c < d);    // 小于
    println!("{} > {} : {}", c, d, c > d);    // 大于
    println!("{} <= {} : {}", c, d, c <= d);  // 小于等于
    println!("{} >= {} : {}", c, d, c >= d);  // 大于等于
    
    // ==================== 逻辑运算符 ====================
    let is_sunny = true;
    let is_warm = false;
    
    println!("\n=== 逻辑运算符 ===");
    println!("is_sunny: {}, is_warm: {}", is_sunny, is_warm);
    println!("!is_sunny = {}", !is_sunny);                    // 逻辑非（取反）
    println!("is_sunny && is_warm = {}", is_sunny && is_warm); // 逻辑与（两个都为真才为真）
    println!("is_sunny || is_warm = {}", is_sunny || is_warm); // 逻辑或（有一个为真就为真）
    
    // ==================== 位运算符 ====================
    let e = 5;   // 二进制: 101
    let f = 3;   // 二进制: 011
    
    println!("\n=== 位运算符 ===");
    println!("{} = {:b} (二进制)", e, e);
    println!("{} = {:b} (二进制)", f, f);
    println!("{} & {} = {} (二进制: {:b})", e, f, e & f, e & f);   // 按位与
    println!("{} | {} = {} (二进制: {:b})", e, f, e | f, e | f);   // 按位或
    println!("{} ^ {} = {} (二进制: {:b})", e, f, e ^ f, e ^ f);   // 按位异或
    println!("!{} = {} (二进制: {:b})", e, !e, !e);                // 按位取反
    println!("{} << 1 = {} (二进制: {:b})", e, e << 1, e << 1);    // 左移
    println!("{} >> 1 = {} (二进制: {:b})", e, e >> 1, e >> 1);    // 右移
    
    // ==================== 赋值运算符 ====================
    let mut g = 10;
    
    println!("\n=== 赋值运算符 ===");
    println!("初始值 g = {}", g);
    
    g += 5;   // 等价于 g = g + 5
    println!("g += 5 后: {}", g);
    
    g -= 3;   // 等价于 g = g - 3
    println!("g -= 3 后: {}", g);
    
    g *= 2;   // 等价于 g = g * 2
    println!("g *= 2 后: {}", g);
    
    g /= 4;   // 等价于 g = g / 4
    println!("g /= 4 后: {}", g);
    
    g %= 3;   // 等价于 g = g % 3
    println!("g %= 3 后: {}", g);
    
    // ==================== 复合类型运算符 ====================
    // 字符串连接
    let first_name = "Merlin";
    let last_name = "QWQ";
    let full_name = format!("{} {}", first_name, last_name);  // 使用 format! 宏
    println!("\n字符串拼接: {}", full_name);
    
    // 数组索引访问
    let numbers = [1, 2, 3, 4, 5];
    println!("数组第一个元素: {}", numbers[0]);  // 通过索引访问
    println!("数组最后一个元素: {}", numbers[4]);
    
    // ==================== 特殊运算符 ====================
    // 范围运算符
    println!("\n=== 范围运算符 ===");
    println!("1..5 生成的范围:");
    for i in 1..5 {  // 不包含 5
        print!("{} ", i);
    }
    println!();
    
    println!("1..=5 生成的范围:");
    for i in 1..=5 {  // 包含 5
        print!("{} ", i);
    }
    println!();
    
    // 引用和解引用运算符
    let h = 42;
    let reference = &h;      // & 是引用运算符
    let value = *reference;  // * 是解引用运算符
    println!("\n原始值: {}, 引用: {:p}, 解引用: {}", h, reference, value);
    
    // match 运算符（模式匹配）
    let grade = 'A';
    let result = match grade {
        'A' => "优秀",
        'B' => "良好",
        'C' => "及格",
        _ => "不及格",
    };
    println!("成绩 {} 对应评价: {}", grade, result);
}