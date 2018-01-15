struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Fibonacci{
    fn new() -> Fibonacci{
        Fibonacci { curr: 1, next: 1 }
    }

}
impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;
        if self.curr > 4000000 {
            return None;
        }
        return Some(self.curr);
    }
}


pub fn execute() {
    let fibonacci : Fibonacci = Fibonacci::new();
    let s:u32 = fibonacci
        .filter(|v| v % 2 == 0)
        .sum();

    println!("sum = {}", s);
}
