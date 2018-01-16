/**

    2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
    What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

pub fn execute() {
    let mut number : u32 = 1;
    loop{
        let mut range = 1..20;
        let count = range.all(|v| number % v == 0);
        if count {
            break;
        }
        number = number + 1;
    }

    println!("number = {}", number);
}
