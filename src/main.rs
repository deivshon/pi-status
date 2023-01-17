pub mod status;

fn main() {
    println!("{}", status::temp::get().unwrap())
}
