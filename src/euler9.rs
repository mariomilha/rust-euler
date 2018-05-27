/**

A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a2 + b2 = c2
For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.

*/

pub fn execute(){
    let pythagorean: Pythagorean = Pythagorean::new();
    let result = pythagorean.filter(|p| p.0 + p.1 + p.2 == 1000)
        .filter(|p| is_square_sum(p.0, p.1, p.2))
        .take(1)
        .next()
        .unwrap();

    println!("result found - {:?}", result);
    println!("result is {}", result.0 * result.1 * result.2);
}

fn is_square_sum(a:u32, b:u32, c:u32) -> bool{
    let sq_a = a * a;
    let sq_b = b * b;
    let sq_c = c * c;
    return (sq_a + sq_b) == sq_c; 
}

struct Pythagorean{
    a:u32,
    b:u32,
    c:u32,
}

impl Pythagorean{
    fn new()-> Pythagorean{
        Pythagorean{a: 1, b: 2, c: 3}
    }
}

impl Iterator for Pythagorean{
    type Item = (u32,u32,u32);

    fn next(&mut self) -> Option<Self::Item>{
        self.c = self.c + 1;
        if !sum_greater_than_thousand(self.a, self.b, self.c) {
            return Some((self.a, self.b, self.c));
        }
        self.b = self.b + 1;
        self.c = self.b + 1;
        if !sum_greater_than_thousand(self.a, self.b, self.c){
            return Some((self.a, self.b, self.c));
        }
        self.a = self.a + 1;
        self.b = self.a + 1;
        if !sum_greater_than_thousand(self.a, self.b, self.c){
            return Some((self.a, self.b, self.c));
        }
        return None;
    }
}

fn sum_greater_than_thousand(a:u32, b:u32, c:u32) -> bool{
    let sum = a + b + c;
    return sum > 1000;
}