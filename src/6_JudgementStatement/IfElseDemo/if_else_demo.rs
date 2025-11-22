fn main() {
    // if、if-else、if-else-if else
    let price:f32 = 222.00;
    if price > 200.00 {
        println!("打八折，共{}元", price*0.8);
    }

    let price:f32 = 110.00;
    if price > 200.00 {
        println!("打八折，共{}元", price*0.8);
    } else {
        println!("原价出售，共{}元", price);
    }

    let price:f32 = 666.00;
    if price > 500.00 {
        println!("打七折，共{}元", price*0.7)
    } else if price <= 500.00 && price > 200.00 {
        println!("打八折，共{}元", price*0.8);
    } else {
        println!("原价出售，共{}元", price);
    } 
}