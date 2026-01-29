use aeternitas::{Event, EventType, Genome};
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
}
