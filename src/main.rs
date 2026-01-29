use aeternitas::{Event, EventType, Genome, Simulacrum};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 1. Instantiate a "Genesis" event
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;

    let genesis_event = Event {
        timestamp: now,
        entity_id: 0,
        event_type: EventType::Genesis,
    };
    println!("{}", genesis_event.to_jsonl());

    // 2. Create "Adam" creature with a random genome
    let adam_genome = Genome::new_random();
    let adam_birth_event = Event {
        timestamp: now + 1, // 1 tick later
        entity_id: 1,       // Adam is ID 1
        event_type: EventType::Birth { genome: adam_genome },
    };
    
    // 3. Print the JSONL log entry to stdout
    println!("{}", adam_birth_event.to_jsonl());

    // 4. Verify Phenotype
    let phenotype = adam_genome.decode();
    println!("DEBUG: Adam's Phenotype: {:?}", phenotype);

    // 5. Initialize the Metabolic State Machine (Simulacrum)
    let mut adam = Simulacrum::new(1, adam_genome);
    
    // 6. Run the Simulation Loop
    println!("\n--- Metabolic Simulation Start ---");
    println!("Initial Energy: {:.2} J | BMR: {:.2} | Mass: {:.2} kg | Max Lifespan: {:.1}", 
             adam.energy, adam.phenotype.bmr, adam.phenotype.body_mass, adam.telomeres);

    let mut senescent_reported = false;

    for tick in 1..=2000 {
        let result = adam.tick();
        
        let is_senescent = result.telomeres <= 0.0;
        let mut status_change = false;

        // Check for Senescence entry
        if is_senescent && !senescent_reported {
            senescent_reported = true;
            status_change = true;
            println!("WARNING: Adam has entered senescence at tick {}!", tick);
        }

        // Print telemetry if: mod 100 == 0 OR status change OR death
        if tick % 100 == 0 || status_change || !result.alive {
            let status_label = if !result.alive {
                "[DEAD]"
            } else if is_senescent {
                "[SENESCENT]"
            } else {
                "[ALIVE]"
            };

            println!(
                "Tick {:4} | Energy: {:8.2} J | Telo: {:6.1} | Status: {}", 
                tick, adam.energy, result.telomeres, status_label
            );
        }
        
        if !result.alive {
            let death_event = Event {
                timestamp: now + tick,
                entity_id: 1,
                event_type: EventType::Death,
            };
            println!("{}", death_event.to_jsonl());
            break;
        }
    }
    println!("--- Metabolic Simulation End ---");
}
