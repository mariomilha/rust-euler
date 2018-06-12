/**
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
*/

pub fn execute() {
    let mut number : u64 = 3;
    let mut primes : Vec<u64> = Vec::new();
    primes.push(2);

    while number < 2_000_000 {
        let prime = check_already_found_primes(number, &primes);
        if prime != 0 {
            println!("Found new prime: {}", prime);
            primes.push(prime);
            println!("primes len = {}", primes.len());
        }
        number = number + 1;
    }
    let result:u64 = primes.iter().sum();
    println!("The sum of all primes is = {}", result);
}

fn check_already_found_primes(to_test:u64, primes: &Vec<u64>) -> u64{
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