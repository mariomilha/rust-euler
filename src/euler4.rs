
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
    let value_iterator = Pair::new();
    let mut result:Option<u32> = value_iterator
        .filter(|v| *v != 0)
        .filter(|v| is_palindrome(v))
        .max();
    let palindrome= result.get_or_insert(1);


    println!("Palindrome = {}", palindrome);
}

fn is_palindrome(value:&u32) -> bool{
    let mut counter : u32 = *value;
    let mut number_of_digits : usize = 0;
    let mut vector:Vec<u32> = vec![0; 7];
    while counter/10 > 0 || counter % 10 != 0 {
        vector[number_of_digits] = counter%10;
        counter = counter /10;
        number_of_digits = number_of_digits +1;
    }
    let mut begin : usize = 0;
    let mut end : usize = number_of_digits -1;
    loop{
        if begin == end || end < begin{
            return true
        }
        if vector[begin] == vector[end]{
            begin = begin + 1;
            end = end -1;
            continue
        }else{
            return false;
        }
    }
}
