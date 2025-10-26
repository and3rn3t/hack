use hack_simulator::challenges::*;

fn main() {
    let all = get_all_challenges();
    println!("Total challenges: {}", all.len());

    for level in 0..=4 {
        let challenges = get_challenges_for_level(level);
        println!("Level {}: {} challenges", level, challenges.len());
        for c in &challenges {
            println!("  - {} ({})", c.id, c.title);
        }
    }
}
