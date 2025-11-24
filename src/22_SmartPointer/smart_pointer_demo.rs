// TODO 后续需要将这个写完，当前只有Box<T>，实际上还有Rc<T>和Arc<T>

fn main() {
    /*
        如果一个结构体实现了deref和drop Trait，它们就不再是简单地结构体了
        Box可以将存储在栈上的数据放到堆上，这叫做装箱
     */
    let a = 6;
    let b = Box::new(a);
    println!("b = {}", b);
}