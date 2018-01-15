
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
