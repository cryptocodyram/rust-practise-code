fn main() {
    let sum = add(100, 200);
    println!("Sum is : {}",sum);
}

fn add(x: i32, y: i32) -> i32 {
    return x+y;
}
