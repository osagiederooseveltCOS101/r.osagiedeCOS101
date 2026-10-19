fn main() {
    let candidates = vec![
        ("Ade", 3),
        ("Kosi", 7),
        ("Chidi", 5),
        ("Roosevelt", 12),
        ("kellogs", 8),
    ];

    let mut highest_years = candidates[0].1;
    let mut highest_name = candidates[0].0;

    let mut i = 0;
    while i < candidates.len() {
        if candidates[i].1 > highest_years {
            highest_years = candidates[i].1;
            highest_name = candidates[i].0;
        }
        i = i + 1;
    }

    println!("Candidate with the highest experience is {} with {} years.", highest_name, highest_years);
}