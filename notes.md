# TODOs

- [ ] Weapon pick up system
- [ ] Spawn and death of characters
- [ ] Endless Road
- [x] Enemy AI
- [x] Walls with fixed body

# Notes
## Hitboxes
### Rapier
- handles collision with `collider` and `CollisionGroups`
- `RigidBody::KinematicVelocityBased` is great for bullets, set direction and velocity, it handles the rest
- scanner for no physics, only collisions
- Characters need `RigidBody::Dynamic` for bullets to collide with characters
  - Have to disable gravity thingies, else it falls.
- 

## Gamepad

- Gamepad is automatically registered and added as a component in bevy
- Query for frame updates (axis) and GamepadEvents for buttons
- `gamepad.rs` gives rough idea, pretty easy 

## Weapons
- `Vec` of `Weapon` is used to track all the weapons of a character
- cooldowns are implemented with a `Timer` and a system which tracks all the weapons by a query through `WeaponCooldown` 
and the `Shootable` trait function `fire_rate`
- `ShootEvent` is being used to track the shots from the gamepad and an `Entity` is saved in the Event

## Base Level Room

- [x] Square room with 2 enemies
- [ ] Shoot both down to start the game menu

## Menu (Game States)

## Map
InfinityMap with branching by applying new map with start of a vertex
