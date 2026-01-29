use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

// --- Part 1: The Genome ---

/// Represents a 512-bit (64-byte) genome.
/// Fixed size structure to ensure predictable memory usage.
#[derive(Debug, Clone, Copy)]
pub struct Genome {
    bytes: [u8; 64],
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
