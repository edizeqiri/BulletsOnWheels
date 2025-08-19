use crate::weapon::Shootable;

#[derive(Debug)]
pub struct Bow{}

impl Shootable for Bow {
    fn shoot(&self) {
        println!("Shooting Bow!")
    }
}
