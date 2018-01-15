
struct Pair{
    x:u32,
    y:u32
}

impl Pair{
    fn new() ->Pair {
        Pair{ x:999, y:999 }
    }
}

impl Iterator for Pair{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item>{
        let to_return = self.x * self.y;
        self.y = self.y -1;
        if self.y == 0 {
            if self.x == 0{
                return None;
            }else{
                self.x = self.x-1;
                self.y = 999;
            }
        }
        return Some(to_return);
    }
}

pub fn execute() {
    let mut value_iterator = Pair::new();
    let mut result:Option<u32> = value_iterator
        .find(|v| is_palindrome(v));
    let palindrome= result.get_or_insert(1);


    println!("Palindrome = {}", palindrome);
}

fn is_palindrome(value:&u32) -> bool{
    //TODO: add logic to discover if the value is a palindrome
    false
}
