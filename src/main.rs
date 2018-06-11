use std::thread;


fn returnFormat(i : i32) -> String{
    format!("{}",i)
}
fn main() {
    println!("{}",returnFormat(30));
}