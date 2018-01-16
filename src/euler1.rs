/**
*
*   If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
*   Find the sum of all the multiples of 3 or 5 below 1000.
*/


pub fn execute() {
    let sequence = 0..1000_i32;
    let value : i32 = sequence
        .filter(|v| v % 3 == 0 || v % 5 == 0)
        .sum();

    println!("sum = {}", value);
}
