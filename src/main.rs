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
    let creature = &world.creatures[0];
    println!("Adam Born. Position: {{x: {}, y: {}}}", creature.pos.x, creature.pos.y);
    println!("Adam Mass: {:.2} kg", creature.phenotype.body_mass);

    // 4. Simulation Loop
    println!("\n--- Starting Simulation (20 Ticks) ---");
    
    for i in 1..=20 {
        world.tick();
        
        // Calculate Total System Energy
        let total_energy: f32 = world.creatures.iter().map(|c| c.energy).sum();
        
        // Print Status (just for the first creature since we only have one)
        // We'll also print total system energy as requested.
        let c = &world.creatures[0];
        println!("Tick {:2} | Time: {:2} | Energy At Pos: {:.2} | Adam Energy: {:.2} | Total System Energy: {:.2}", 
                 i, world.time, world.energy_at(c.pos), c.energy, total_energy);
    }
}
