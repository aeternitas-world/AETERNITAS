use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// --- Part 1: The Genome ---

/// Simple Linear Congruential Generator (LCG) for deterministic randomness.
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        const A: u64 = 6364136223846793005;
        const C: u64 = 1442695040888963407;
        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }

    /// Helper for probabilities, returns 0.0..1.0
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u64() >> 32) as f32 / u32::MAX as f32
    }
}

/// Represents a 512-bit (64-byte) genome.
#[derive(Debug, Clone, Copy)]
pub struct Genome {
    bytes: [u8; 64],
}

/// The expressed traits of a creature.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Phenotype {
    pub bmr: f32,               // Metabolic Rate
    pub body_mass: f32,         // Mass in kg
    pub perception_radius: f32, // Perception range
    pub max_lifespan: f32,      // Max lifespan
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn dist(&self, other: &Position) -> f32 {
        let dx = self.x as f32 - other.x as f32;
        let dy = self.y as f32 - other.y as f32;
        (dx * dx + dy * dy).sqrt()
    }
}

impl Genome {
    pub fn new_random() -> Self {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_nanos() as u64,
            Err(_) => 42,
        };
        Self::from_seed(seed)
    }

    pub fn from_seed(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut bytes = [0u8; 64];
        for i in 0..64 {
            let r = rng.next_u64();
            bytes[i] = (r >> 56) as u8;
        }
        Genome { bytes }
    }

    pub fn crossover(&self, other: &Genome, rng: &mut Rng) -> Genome {
        let mut child_bytes = [0u8; 64];
        for i in 0..8 {
            let offset = i * 8;
            let range = offset..offset+8;
            let self_chunk = u64::from_ne_bytes(self.bytes[range.clone()].try_into().unwrap());
            let other_chunk = u64::from_ne_bytes(other.bytes[range.clone()].try_into().unwrap());
            let mask = rng.next_u64();
            let child_chunk = (self_chunk & mask) | (other_chunk & !mask);
            child_bytes[range].copy_from_slice(&child_chunk.to_ne_bytes());
        }
        Genome { bytes: child_bytes }
    }

    pub fn mutate(&mut self, rng: &mut Rng) {
        for byte in self.bytes.iter_mut() {
            for bit in 0..8 {
                if rng.next_f32() < 0.0001 {
                    *byte ^= 1 << bit;
                }
            }
        }
    }

    pub fn decode(&self) -> Phenotype {
        let get_u32 = |start: usize| -> u32 {
            let slice: [u8; 4] = self.bytes[start..start + 4].try_into().expect("Slice error");
            u32::from_le_bytes(slice)
        };
        
        let gray_decode = |mut n: u32| -> u32 {
            let mut p = n;
            while p > 0 { p >>= 1; n ^= p; }
            n
        };
        
        let normalize = |v: u32| -> f32 { v as f32 / u32::MAX as f32 };

        let norm_bmr = normalize(gray_decode(get_u32(0)));
        let norm_mass = normalize(gray_decode(get_u32(4)));
        let norm_perception = normalize(gray_decode(get_u32(16)));
        let norm_lifespan = normalize(gray_decode(get_u32(34)));

        Phenotype {
            bmr: norm_bmr * (2.0 - 0.5) + 0.5,
            body_mass: norm_mass * (100.0 - 1.0) + 1.0,
            perception_radius: norm_perception * (100.0 - 1.0) + 1.0,
            max_lifespan: norm_lifespan * (5000.0 - 1000.0) + 1000.0,
        }
    }
}

impl fmt::Display for Genome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.bytes {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

// --- Part 2: The Event Log ---

#[derive(Debug)]
pub enum Event {
    Birth { parent_id: u64 },
    Death { id: u64, reason: String },
    Move { id: u64, x: u16, y: u16 },
}

impl Event {
    pub fn to_jsonl(&self) -> String {
        match self {
            Event::Birth { parent_id } => format!(r#"{{"type":"Birth","parent_id":{}}}"#, parent_id),
            Event::Death { id, reason } => format!(r#"{{"type":"Death","id":{},"reason":"{}"}}"#, id, reason),
            Event::Move { id, x, y } => format!(r#"{{"type":"Move","id":{},"x":{},"y":{}}}"#, id, x, y),
        }
    }
}

// --- Part 3: The Simulation ---

#[derive(Debug)]
pub struct Simulacrum {
    pub id: u64,
    pub genome: Genome,
    pub phenotype: Phenotype,
    pub pos: Position,
    pub energy: f32,
    pub alive: bool,
}

impl Simulacrum {
    pub fn new(id: u64, genome: Genome, start_pos: Position) -> Self {
        let phenotype = genome.decode();
        Simulacrum {
            id,
            genome,
            phenotype,
            pos: start_pos,
            energy: 100.0, // Initial energy buffer
            alive: true,
        }
    }

    pub fn move_to(&mut self, target: Position, world_size: u16) -> Option<Event> {
        if target.x >= world_size || target.y >= world_size {
            return None;
        }

        let dist = self.pos.dist(&target);
        // Energy Cost: 0.5 * Mass * Dist^2 * C_drag (0.1)
        let cost = 0.5 * self.phenotype.body_mass * dist.powi(2) * 0.1;

        self.energy -= cost;
        self.pos = target;

        Some(Event::Move { id: self.id, x: self.pos.x, y: self.pos.y })
    }
}

pub struct World {
    pub size: u16,
    pub creatures: Vec<Simulacrum>,
    pub tick_count: u64,
    pub rng: Rng,
    pub next_id: u64,
}

impl World {
    pub fn new(size: u16, seed: u64) -> Self {
        World {
            size,
            creatures: Vec::new(),
            tick_count: 0,
            rng: Rng::new(seed),
            next_id: 1,
        }
    }

    /// Static calculation of energy to avoid borrow checker issues with `self`.
    pub fn calculate_energy(tick: u64, pos: Position) -> f32 {
        let t = tick as f32 * 0.01;
        let x = pos.x as f32 * 0.1;
        let y = pos.y as f32 * 0.1;
        
        let pattern = (t + x).sin() * (t + y).cos();
        let norm = (pattern + 1.0) / 2.0;
        
        // Base 1.0 + Variable up to 10.0
        1.0 + (10.0 * norm)
    }

    pub fn energy_at(&self, pos: Position) -> f32 {
        Self::calculate_energy(self.tick_count, pos)
    }

    pub fn tick(&mut self) -> Vec<Event> {
        self.tick_count += 1;
        let current_tick = self.tick_count; // Capture for use in closure/loop
        let mut events = Vec::new();
        
        let mut dead_indices = Vec::new();
        let mut repro_ids = Vec::new(); // Use IDs to track reproduction parents safely

        // Pass 1: Analysis & Action
        for (i, creature) in self.creatures.iter_mut().enumerate() {
            if !creature.alive {
                dead_indices.push(i);
                continue;
            }

            // 1. Gain (Use static helper to avoid borrowing &self)
            let gain = Self::calculate_energy(current_tick, creature.pos);
            
            // 2. Cost
            let loss = creature.phenotype.bmr;
            
            // 3. Apply
            creature.energy += gain - loss;

            // 4. Action: Attempt Random Move
            let r1 = self.rng.next_u64();
            let r2 = self.rng.next_u64();
            let dx = (r1 % 3) as i32 - 1; 
            let dy = (r2 % 3) as i32 - 1;
            
            if dx != 0 || dy != 0 {
                let tx = (creature.pos.x as i32 + dx).max(0).min(self.size as i32 - 1) as u16;
                let ty = (creature.pos.y as i32 + dy).max(0).min(self.size as i32 - 1) as u16;
                
                if let Some(_evt) = creature.move_to(Position { x: tx, y: ty }, self.size) {
                    // events.push(evt); // Not strictly required by prompt output, but good for debug
                }
            }

            // 5. Check Vitals
            if creature.energy <= 0.0 {
                dead_indices.push(i);
                continue; 
            }

            // 6. Check Reproduction
            let threshold = 50.0; // Adjusted threshold
            // Ensure creature has enough energy to split (e.g. at least > cost)
            if creature.energy > threshold {
                 repro_ids.push(creature.id);
            }
        }

        // Pass 2: Process Deaths (Cleanup)
        dead_indices.sort_by(|a, b| b.cmp(a)); // Descending
        dead_indices.dedup();

        for index in dead_indices {
            if index < self.creatures.len() {
                let id = self.creatures[index].id;
                self.creatures.remove(index);
                events.push(Event::Death { id, reason: "Energy Depletion".to_string() });
            }
        }

        // Pass 3: Process Births (Growth)
        // Note: Using IDs is O(N^2) here but safe and N is small (100).
        let mut offspring = Vec::new();
        
        for p_id in repro_ids {
            // Find parent index
            // We use iter_mut because we need to modify parent energy AND read genome
            if let Some(parent) = self.creatures.iter_mut().find(|c| c.id == p_id) {
                 // Double check parent is still valid/alive/has energy?
                 // They should be, unless they died? No, specific logic excludes dead from repro_ids.
                 // But wait, if died in move? No, checked after move and before repro add.
                 
                 let split_cost = 25.0; // Cost to birth
                 if parent.energy > split_cost {
                     parent.energy -= split_cost;
                     
                     let mut child_genome = parent.genome.clone();
                     child_genome.mutate(&mut self.rng);
                     
                     let child_id = self.next_id;
                     self.next_id += 1;
                     
                     let child = Simulacrum {
                         id: child_id,
                         genome: child_genome,
                         phenotype: child_genome.decode(),
                         pos: parent.pos, // Start at parent location
                         energy: split_cost,
                         alive: true,
                     };
                     
                     offspring.push(child);
                     events.push(Event::Birth { parent_id: parent.id });
                 }
            }
        }
        
        self.creatures.append(&mut offspring);

        events
    }
}
