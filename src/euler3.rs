/**

    The prime factors of 13195 are 5, 7, 13 and 29.
    What is the largest prime factor of the number 600851475143 ?
*/
pub fn execute() {
    let mut number: u64 = 600851475143;
    let mut divisor: u64 = 2;
    while number > 1 {
        if number % divisor != 0{
            divisor = divisor +1;
        }else{
            number = number/divisor;
        }
    }
    println!("max prime = {}", divisor);
}
