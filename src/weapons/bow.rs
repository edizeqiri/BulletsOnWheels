struct Bow{}

impl Shootable for Bow {
    fn shoot(&self) {
        println!("Shooting Bow!")
    }
}
