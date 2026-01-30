use aeternitas::{Genome, Position, Simulacrum, World, Rng};

fn main() {
    // 1. Initialize World
    // Size 100, Seed 42 for determinism
    let mut world = World::new(100, 42);

    // 2. Initialize 100 Random Creatures
    // Use a deterministic RNG for setup (independent of World's internal RNG to keep setup clean)
    let mut init_rng = Rng::new(12345);

    for _ in 0..100 {
        let genome_seed = init_rng.next_u64();
        let genome = Genome::from_seed(genome_seed);
        
        // Random Position
        let x = (init_rng.next_u64() % 100) as u16;
        let y = (init_rng.next_u64() % 100) as u16;
        let pos = Position { x, y };

        // Create Creature
        let id = world.next_id;
        world.next_id += 1;
        
        let creature = Simulacrum::new(id, genome, pos);
        world.creatures.push(creature);
    }

    // 3. Execution Loop
    println!("Starting AETERNITAS Simulation...");
    println!("---------------------------------");

    // Run for 100 ticks
    for _ in 0..100 {
        let _events = world.tick();
        
        // Print Summary: Tick: X | Pop: Y
        println!("Tick: {:3} | Pop: {:4}", world.tick_count, world.creatures.len());
    }
    
    println!("---------------------------------");
    println!("Simulation Complete.");
}
