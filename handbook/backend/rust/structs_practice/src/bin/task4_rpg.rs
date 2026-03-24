/// Task 4: RPG character (multiple structs).

struct Weapon {
    name: String,
    damage: u32,
}

struct Character {
    name: String,
    health: u32,
}

impl Character {
    /// Reduces health by the given damage (clamps at 0).
    fn take_damage(&mut self, damage: u32) {
        // TODO: Implement clamped health reduction.
        let _ = damage;
    }
}

fn attack(weapon: &Weapon, target: &mut Character) {
    // TODO: Implement applying weapon damage to the character.
    let _ = weapon;
    let _ = target;
}

fn main() {
    let mut hero = Character {
        name: String::from("Hero"),
        health: 100,
    };
    let sword = Weapon {
        name: String::from("Sword"),
        damage: 35,
    };

    attack(&sword, &mut hero);

    println!("{} remaining health: {}", hero.name, hero.health);
}

