use bevy::prelude::{Bundle, Component, Sprite, Transform};
use crate::{Bow, Enemy, Player};


#[derive(Component)]
enum Weapon {
    Bow(Bow)
}

trait Shootable {
    fn shoot(&self);
}

