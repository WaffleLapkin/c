#![forbid(unsafe_code)]

fn main() {
    let time = chrono::offset::Local::now();
    println!("{}", time);
}
