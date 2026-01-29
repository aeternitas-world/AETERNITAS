use aeternitas::{Genome, Position, Simulacrum, World};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 1. Setup Randomness (for potential future use, keeping consistent with style)
    let _seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;

    println!("--- Permacomputing Spatial Grid Demo ---");

    // 2. Initialize World
    let mut world = World::new(100);
    println!("Initialized World (Size: {})", world.size);

    // 3. Place 'Adam' at {50, 50}
    // Note: We need a genome for Adam.
    let adam_genome = Genome::new_random();
    let start_pos = Position { x: 50, y: 50 };
    let adam = Simulacrum::new(1, adam_genome, start_pos);
    
    world.creatures.push(adam);
    
    // We access Adam from the world to simulate the loop
    let creature = &mut world.creatures[0];
    println!("Adam Born. Position: {{x: {}, y: {}}}", creature.pos.x, creature.pos.y);
    println!("Adam Mass: {:.2} kg", creature.phenotype.body_mass);

    // 4. Move to {51, 50}
    let target_pos = Position { x: 51, y: 50 };
    let timestamp = 1; // Simulation tick 1

    println!("\n--- Attempting Move ---");
    match creature.move_to(target_pos, world.size, timestamp) {
        Some((cost, event)) => {
            println!("Move Successful.");
            println!("New Location: {{x: {}, y: {}}}", creature.pos.x, creature.pos.y);
            println!("Energy Cost: {:.4} J", cost);
            println!("Remaining Energy: {:.4} J", creature.energy);
            println!("Log: {}", event.to_jsonl());
        },
        None => {
            println!("Move Failed: Target out of bounds.");
        }
    }
}
