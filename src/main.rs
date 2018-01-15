use std::collections::LinkedList;

fn main() {
    println!("Hello, world!");
    let number = 1000_i32;
    let mut list : LinkedList<i32> = LinkedList::new();
    for cur in 1..number {
        if(cur % 3) == 0 || (cur % 5) == 0{
            list.push_back(cur);
        }
    }

    let mut it = list.iter();
    loop{
        let mut elem :Option<&i32> = it.next();
        let x = *elem.get_or_insert(&-1);
        if *x ==-1 {
            break;
        }
        println!("x = {}", x);
    }
}
