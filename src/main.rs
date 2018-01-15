
fn main() {
    let sequence = 0..10000_i32;
    let values = sequence
        .filter(|v| v % 3 == 0 || v % 5 == 0)
        .collect::<Vec<i32>>();

    for x in values.iter(){
        println!("X = {}", x);
    }
}
