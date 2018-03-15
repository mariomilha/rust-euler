/**

    By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
    What is the 10 001st prime number?
*/

pub fn execute() {
    let mut number : u32 = 3;
    let mut primes : Vec<u32> = Vec::new();
    primes.push(2);

    while primes.len() < 10_001 {
        let prime = check_already_found_primes(number, &primes);
        if prime != 0 {
//            println!("Found new prime: {}", prime);
            primes.push(prime);
            println!("primes len = {}", primes.len());
        }
        number = number + 1;
    }

    println!("10 001 prime is = {}", primes[primes.len() - 1]);
}

fn check_already_found_primes(to_test:u32, primes: &Vec<u32>) -> u32{
    if to_test % 2 == 0 {
        return 0;
    }
    for prime in primes {
        if to_test % prime == 0{
            return 0;
        }
    }
    let mut last_number = primes[primes.len() - 1];
    while last_number < to_test {
        if to_test % last_number == 0{
            return 0;
        }
        last_number = last_number + 1;
    }
    return last_number;
}

