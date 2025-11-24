fn main() {
    /*
        | 输入参数（可能多个） | { 表达式 };

        闭包是在一个函数内部创建并理解调用的函数，没有函数签名，是一个匿名函数
        闭包不用声明返回值，但是可以有返回值
        闭包有能力捕获外部环境的变量
        闭包可以再没有类型标注的情况下运行，可以move，也可以borrow，意味着他可以捕获&T、&mut T、T
    */

    let double = |x|{x*2};
    let add = |a, b|{a + b};
    let x = add(2, 4);
    println!("{}", x);

    let y = double(5);
    println!("{}", y);

    // add2捕获了外部环境中的v这个变量
    let v = 3;
    let add2 = |x|{v + x};
    println!("{}", add2(4));
}