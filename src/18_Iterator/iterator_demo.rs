fn main() {
    /*
        iter() 返回一个只读迭代器，原集合依然可以使用，迭代器元素类型为&T
        into_iter() 返回一个只读迭代器，但是原集合不可再次使用了，迭代器元素为T
        iter_mut() 返回一个可修改的迭代器，并且原集合依旧可以使用，迭代器元素的类型为&mut T
     */
    let v = vec!["Alice", "Bob", "Eva"];
    let mut it = v.iter();
    // next()返回迭代器的下一个元素
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());
    println!("{:?}", it.next());    // 遍历结束则是None
}