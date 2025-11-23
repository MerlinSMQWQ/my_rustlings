fn main() {
    let student_list = vec!["Alice", "Bob", "Cline"];
    let mut student_list2 = student_list;
    show(&student_list2);
    println!("{:?}", student_list2);
    show2(&mut student_list2);
    println!("{:?}", student_list2);
}
// 不可变借用
fn show(v: &Vec<&str>) {
    println!("{:?}", v);
}

// 可变借用
fn show2(v: &mut Vec<&str>) {
    v[0] = "Eva";
}