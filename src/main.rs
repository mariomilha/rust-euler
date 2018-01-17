extern crate time;

mod euler6;

fn main() {
   let start_time = time::get_time().nsec;
   println!("start running");
   euler6::execute();
   println!("finished in - {} nano seconds", time::get_time().nsec - start_time);
}
