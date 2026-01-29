use aeternitas::{Genome, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 1. Setup Randomness
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;
    let mut rng = Rng::new(seed);

    println!("--- Sexual Reproduction Demo ---");

    // 2. Create Parents: "Adam" and "Eve"
    let adam = Genome::new_random();
    let eve = Genome::new_random();

    println!("Parent A (Adam): {}", adam);
    println!("Parent B (Eve):  {}", eve);

    // 3. Perform Crossover to create "Cain"
    let mut cain = adam.crossover(&eve, &mut rng);
    
    // Capture pre-mutation state for comparison (optional, but interesting)
    // println!("Cain (Pre-Mut):  {}", cain);

    // 4. Apply Mutation to "Cain"
    cain.mutate(&mut rng);

    // 5. Print all three Genomes
    println!("Child (Cain):    {}", cain);
    
    println!("\n--- Analysis ---");
    // Verify differentiation
    if adam.to_string() != eve.to_string() {
        println!("Parents are unique.");
    }
    if cain.to_string() != adam.to_string() && cain.to_string() != eve.to_string() {
        println!("Child is unique from parents.");
    } else {
        println!("Child is identical to one parent (possible due to crossover luck or identical parents).");
    }
}
