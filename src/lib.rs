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
        // Use high 32 bits for better randomness
        (self.next_u64() >> 32) as f32 / u32::MAX as f32
    }
}

/// Represents a 512-bit (64-byte) genome.
/// Fixed size structure to ensure predictable memory usage.
#[derive(Debug, Clone, Copy)]
pub struct Genome {
    bytes: [u8; 64],
}

/// The expressed traits of a creature, derived from its Genome.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Phenotype {
    pub bmr: f32,               // Metabolic Rate [0.5, 2.0]
    pub body_mass: f32,         // Mass in kg [1.0, 100.0]
    pub perception_radius: f32, // Perception range in meters [1.0, 100.0]
    pub max_lifespan: f32,      // Max lifespan in ticks [1000.0, 5000.0]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    /// Calculates Euclidean distance between two positions.
    pub fn dist(&self, other: &Position) -> f32 {
        let dx = self.x as f32 - other.x as f32;
        let dy = self.y as f32 - other.y as f32;
        (dx * dx + dy * dy).sqrt()
    }
}

impl Genome {
    /// Creates a new random genome using a simple Linear Congruential Generator (LCG).
    /// Uses the current system time as a seed.
    pub fn new_random() -> Self {
        let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_nanos() as u64,
            Err(_) => 42,
        };
        
        let mut rng = Rng::new(seed);
        let mut bytes = [0u8; 64];

        // Fill bytes using the RNG
        for i in 0..64 {
            // Using the LCG to generate bytes. 
            // We can grab chunks or just take the high byte of each generation to match previous logic logic
            // (previous logic: byte = (seed >> 56) as u8)
            // rng.next_u64() updates state and returns it.
            let r = rng.next_u64();
            bytes[i] = (r >> 56) as u8;
        }

        Genome { bytes }
    }

    /// Combines this genome with another using Uniform Crossover.
    /// 50% chance per bit to inherit from Parent A or B.
    /// Optimized by processing 64-bit chunks.
    pub fn crossover(&self, other: &Genome, rng: &mut Rng) -> Genome {
        let mut child_bytes = [0u8; 64];
        
        // Iterate as 64-bit chunks (8 chunks * 8 bytes = 64 bytes)
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

    /// Applies point mutations to the genome.
    /// Mutation rate: 0.0001 per bit.
    pub fn mutate(&mut self, rng: &mut Rng) {
        for byte in self.bytes.iter_mut() {
            for bit in 0..8 {
                if rng.next_f32() < 0.0001 {
                    *byte ^= 1 << bit;
                }
            }
        }
    }

    /// Decodes the genome into phenotypic traits using Gray Code to Binary conversion.
    ///
    /// Mappings:
    /// - Bits 0-31:   Metabolism (BMR)       => [0.5, 2.0]
    /// - Bits 32-63:  Morphology (Body Mass) => [1.0, 100.0] kg
    /// - Bits 128-159: Sensory (Perception)  => [1.0, 100.0] m (Using lower 32 bits of 128-191 block)
    /// - Bits 272-303: Regulatory (Lifespan) => [1000.0, 5000.0] ticks
    pub fn decode(&self) -> Phenotype {
        // Helper to extract u32 from bytes (Little Endian)
        let get_u32 = |start: usize| -> u32 {
            let slice: [u8; 4] = self.bytes[start..start + 4]
                .try_into()
                .expect("Slice extraction failed");
            u32::from_le_bytes(slice)
        };

        // 1. Extract raw integers
        let raw_bmr = get_u32(0);      // Bits 0-31
        let raw_mass = get_u32(4);     // Bits 32-63
        let raw_perception = get_u32(16); // Bits 128-159 (Start of 128-191 block)
        let raw_lifespan = get_u32(34); // Bits 272-303 (Byte 34-37)

        // 2. Gray Decode (converts Gray code to Binary)
        let bin_bmr = gray_decode(raw_bmr);
        let bin_mass = gray_decode(raw_mass);
        let bin_perception = gray_decode(raw_perception);
        let bin_lifespan = gray_decode(raw_lifespan);

        // 3. Normalize and Scale
        // Maps u32 [0, MAX] -> f32 [0.0, 1.0]
        let normalize = |v: u32| -> f32 {
            v as f32 / u32::MAX as f32
        };

        let norm_bmr = normalize(bin_bmr);
        let norm_mass = normalize(bin_mass);
        let norm_perception = normalize(bin_perception);
        let norm_lifespan = normalize(bin_lifespan);

        Phenotype {
            bmr: norm_bmr * (2.0 - 0.5) + 0.5,
            body_mass: norm_mass * (100.0 - 1.0) + 1.0,
            perception_radius: norm_perception * (100.0 - 1.0) + 1.0,
            max_lifespan: norm_lifespan * (5000.0 - 1000.0) + 1000.0,
        }
    }
}

/// Converts a 32-bit Reflective Binary Gray Code (RBGC) to a standard binary integer.
/// 
/// See: https://en.wikipedia.org/wiki/Gray_code#Converting_to_and_from_Gray_code
fn gray_decode(mut n: u32) -> u32 {
    let mut p = n;
    while p > 0 {
        p >>= 1;
        n ^= p;
    }
    n
}

/// Formats the genome as a hexadecimal string.
impl fmt::Display for Genome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in &self.bytes {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

// --- Part 2: The Event Log (AOL) ---

/// Defines the type of event in the biosphere.
#[derive(Debug)]
pub enum EventType {
    Genesis,
    Birth { genome: Genome },
    Death,
    Move { x: u16, y: u16 },
}

/// Represents a discrete moment in history.
#[derive(Debug)]
pub struct Event {
    pub timestamp: u64,
    pub entity_id: u64,
    pub event_type: EventType,
}

impl Event {
    /// Manually formats the event as a JSON line string.
    /// No serde, no external libs.
    pub fn to_jsonl(&self) -> String {
        let type_str = match &self.event_type {
            EventType::Genesis => format!(r#""type":"Genesis""#),
            EventType::Birth { genome } => format!(r#""type":"Birth","genome":"{}""#, genome),
            EventType::Death => format!(r#""type":"Death""#),
            EventType::Move { x, y } => format!(r#""type":"Move","x":{},"y":{}"#, x, y),
        };

        format!(
            r#"{{"timestamp":{},"entity_id":{},{}}}"#,
            self.timestamp, self.entity_id, type_str
        )
    }
}

// --- Part 3: The Metabolic State Machine ---

/// Represents the result of a single simulation tick.
#[derive(Debug, Clone, Copy)]
pub struct TickResult {
    pub energy_spent: f32,
    pub alive: bool,
    pub telomeres: f32,
}

/// A living entity in the simulation.
#[derive(Debug)]
pub struct Simulacrum {
    pub id: u64,
    pub genome: Genome,
    pub phenotype: Phenotype,
    pub pos: Position,
    pub energy: f32, // Joules
    pub telomeres: f32, // Telomere Counter (T_elo)
    pub alive: bool,
}

impl Simulacrum {
    /// Creates a new creature with a starting energy buffer.
    pub fn new(id: u64, genome: Genome, start_pos: Position) -> Self {
        let phenotype = genome.decode();
        Simulacrum {
            id,
            genome,
            phenotype,
            pos: start_pos,
            energy: 1000.0, // Starting buffer so it doesn't die instantly
            telomeres: phenotype.max_lifespan,
            alive: true,
        }
    }

    /// Calculates the Basal Metabolic Rate (BMR) using a simplified Kleiber's Law.
    /// Formula: BMR * Mass^0.75
    pub fn calculate_bmr(phenotype: &Phenotype) -> f32 {
        phenotype.bmr * phenotype.body_mass.powf(0.75)
    }

    /// Advances the creature's state by one tick.
    /// Consumes energy based on BMR. Dies if energy <= 0.
    pub fn tick(&mut self) -> TickResult {
        if !self.alive {
            return TickResult {
                energy_spent: 0.0,
                alive: false,
                telomeres: self.telomeres,
            };
        }

        // Decrement Telomeres
        self.telomeres -= 1.0;

        // Calculate Cost
        let mut cost = Self::calculate_bmr(&self.phenotype);

        // Apply Senescence Penalty
        if self.telomeres <= 0.0 {
            cost *= 1.5;
        }

        self.energy -= cost;

        if self.energy <= 0.0 {
            self.energy = 0.0;
            self.alive = false;
        }

        TickResult {
            energy_spent: cost,
            alive: self.alive,
            telomeres: self.telomeres,
        }
    }

    /// Moves the creature to a target position.
    /// Consumes energy based on distance and body mass.
    pub fn move_to(&mut self, target: Position, world_size: u16, timestamp: u64) -> Option<(f32, Event)> {
        // Check bounds (Closed Grid)
        if target.x >= world_size || target.y >= world_size {
            return None;
        }

        let dist = self.pos.dist(&target);
        
        // Energy Cost Model: Cost = Dist * Mass * 0.01 (Friction/Efficiency)
        let cost = dist * self.phenotype.body_mass * 0.01;

        self.energy -= cost;
        self.pos = target;

        let event = Event {
            timestamp,
            entity_id: self.id,
            event_type: EventType::Move { x: self.pos.x, y: self.pos.y },
        };

        Some((cost, event))
    }
}

/// The Simulation World State
pub struct World {
    pub size: u16,
    pub creatures: Vec<Simulacrum>,
}

impl World {
    pub fn new(size: u16) -> Self {
        World {
            size,
            creatures: Vec::new(),
        }
    }
}
