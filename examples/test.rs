use rust_units::{Quantity, si_system::dimensions::*};

fn main() {
    let m: Quantity<_, Mass> = Quantity::from_work(42.);
    let v: Quantity<_, Volume> = Quantity::from_work(3.);

    println!("{m}");
    println!("{v}");
    println!("{}", m/v);
}