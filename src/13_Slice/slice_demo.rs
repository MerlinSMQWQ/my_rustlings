fn main() {
    let mut v = Vec::new();
    v.push("Merlin");
    v.push("QWQ");

    println!("len: {}", v.len());

    // 可变切片（不可变切片没有mut）
    let s1 = &mut v[0..=1];
    println!("s1: {:?}", s1);
    // change(&mut v)   这种写法是错误的，因为rust不允许一次性借用两次
    change(s1);
    println!("s1: {:?}", s1);
}

fn change(s: &mut [&str]) {
    s[0] = "merlin";
    println!("changed s: {:?}", s);
}