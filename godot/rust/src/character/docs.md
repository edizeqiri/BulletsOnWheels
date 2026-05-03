# Character
## Enemy

- Enemies will follow the `Transform` of `Player` by substracting the player vector with their own.
    - The same for aiming
- Added 20% Speed reduction for movement to not copy the moves
- Enemies shoot by creating `ShootEvent`. Thus, they behave the same as a player.
