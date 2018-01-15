
fn main() {
    let sequence = 0..1000_i32;
    let value : i32 = sequence
        .filter(|v| v % 3 == 0 || v % 5 == 0)
        .sum();

    println!("sum = {}", value);
}
