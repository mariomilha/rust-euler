/**

    The sum of the squares of the first ten natural numbers is,
    12 + 22 + ... + 102 = 385
    The square of the sum of the first ten natural numbers is,
    (1 + 2 + ... + 10)2 = 552 = 3025
    Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
    Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
*/

pub fn execute() {
    let first_range = 1..101;
    let sum : u64 = first_range.sum();
    let squared_sum = square(sum);
    println!("squared_sum = {}", squared_sum);

    let second_range = 1..101;
    let sum_of_squares:u64 = second_range
        .map(|v| square(v))
        .sum();
    println!("sum_of_squares = {}", sum_of_squares);

    println!("squared_sum - sum_of_squares = {}", squared_sum - sum_of_squares);
}

fn square(value:u64) -> u64{
    let square = value * value;
    println!("v = {}, square = {}", value, square);
    return square;
}
