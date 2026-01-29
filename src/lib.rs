use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// --- Part 1: The Genome ---

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
}

impl Genome {
    /// Creates a new random genome using a simple Linear Congruential Generator (LCG).
    /// Uses the current system time as a seed.
    ///
    /// Algorithm: x_{n+1} = (a * x_n + c) % m
    /// We use constants from PCG or similar reliable LCGs, adapted for u64.
    pub fn new_random() -> Self {
        let mut seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_nanos() as u64,
            Err(_) => 42, // Fallback seed if time fails (unlikely)
        };

        // LCG Constants (from MMIX by Knuth)
        const A: u64 = 6364136223846793005;
        const C: u64 = 1442695040888963407;

        let mut bytes = [0u8; 64];

        for i in 0..64 {
            seed = seed.wrapping_mul(A).wrapping_add(C);
            // Use the high bits for better randomness quality in simple LCGs
            bytes[i] = (seed >> 56) as u8; 
        }

        Genome { bytes }
    }

    /// Decodes the genome into phenotypic traits using Gray Code to Binary conversion.
    ///
    /// Mappings:
    /// - Bits 0-31:   Metabolism (BMR)       => [0.5, 2.0]
    /// - Bits 32-63:  Morphology (Body Mass) => [1.0, 100.0] kg
    /// - Bits 128-159: Sensory (Perception)  => [1.0, 100.0] m (Using lower 32 bits of 128-191 block)
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

        // 2. Gray Decode (converts Gray code to Binary)
        let bin_bmr = gray_decode(raw_bmr);
        let bin_mass = gray_decode(raw_mass);
        let bin_perception = gray_decode(raw_perception);

        // 3. Normalize and Scale
        // Maps u32 [0, MAX] -> f32 [0.0, 1.0]
        let normalize = |v: u32| -> f32 {
            v as f32 / u32::MAX as f32
        };

        let norm_bmr = normalize(bin_bmr);
        let norm_mass = normalize(bin_mass);
        let norm_perception = normalize(bin_perception);

        Phenotype {
            bmr: norm_bmr * (2.0 - 0.5) + 0.5,
            body_mass: norm_mass * (100.0 - 1.0) + 1.0,
            perception_radius: norm_perception * (100.0 - 1.0) + 1.0,
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
    Move { x: i32, y: i32 },
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
