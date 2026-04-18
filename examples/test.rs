use rust_units::{Quantity, si_system::*};

fn main() {
    let m: Quantity<_, Mass> = Quantity::from_si(42.);
    let v: Quantity<_, Volume> = Quantity::from_si(3.);

    println!("{m}");
    println!("{v}");
    println!("{}", m/v);
}