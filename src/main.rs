use aeternitas::{Genome, Position, Simulacrum, World};


fn main() {
    // 1. Setup Randomness
    // We use a fixed seed for the world to ensure deterministic execution as requested, 
    // but here we can generate one if we wanted. 
    // "Maintain strictly deterministic execution order." -> I'll use a fixed seed.
    let seed = 42;

    println!("--- Permacomputing Spatial Grid Demo ---");

    // 2. Initialize World
    let mut world = World::new(100, seed);
    println!("Initialized World (Size: {}, Seed: {})", world.size, seed);

    // 3. Place 'Adam' at {50, 50}
    // We use the same seed for Adam's genome to ensure complete determinism of the run.
    let adam_genome = Genome::from_seed(seed); 
    
    let start_pos = Position { x: 50, y: 50 };
    // We use ID 0 for Adam. World starts IDs at 1.
    let adam = Simulacrum::new(0, adam_genome, start_pos);
    
    world.creatures.push(adam);
    
    // We access Adam from the world to simulate the loop
    {
        let creature = &world.creatures[0];
        println!("Adam Born. Position: {{x: {}, y: {}}}", creature.pos.x, creature.pos.y);
        println!("Adam Mass: {:.2} kg", creature.phenotype.body_mass);
    } // End borrow

    // 4. Simulation Loop
    println!("\n--- Starting Simulation (100 Ticks) ---");
    
    for _ in 1..=100 {
        let events = world.tick();
        
        let pop_size = world.creatures.len();
        
        // Print Summary for the tick
        println!("Tick {:3} | Pop: {:3} | Events: {}", 
                 world.tick_count, pop_size, events.len());

        // Print Events Detail
        for event in events {
            println!("  -> {}", event.to_jsonl());
        }
        
        // If population is dead, stop? Or run emptiness?
        if pop_size == 0 {
            println!("Extinction Event happened at tick {}", world.tick_count);
            break;
        }
    }
}
