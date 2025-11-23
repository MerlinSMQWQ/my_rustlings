fn main() {
    /*
        Vec
        1. 相同数据类型的元素的集合
        2. 长度可变，运行时可以增加或者减少
        3. 使用索引查找
        4. 添加元素至队尾
        5. 在堆上存储，长度可以动态变化

        创建Vec的方法：
            Vec::new();
            vec![val1, val2, ...]
     */

    let mut v: Vec<&str> = Vec::new();
    // push方法可以将元素添加到Vec队尾
    v.push("Hello");
    v.push("World");
    v.push("!");
    println!("{:?}", v);
    // remove方法可以从Vec移除指定索引的元素，并返回这个元素
    let x = v.remove(2);
    println!("{}", x);
    println!("{:?}", v);

    for item in v.iter() {
        println!("{}", item);
    }
}