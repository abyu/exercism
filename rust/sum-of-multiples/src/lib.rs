pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut values = (1..limit)
        .flat_map(|v| factors.iter().map(move |factor| factor * v))
        .filter(|&v| v < limit)
        .collect::<Vec<u32>>();

    values.sort();
    values.dedup();

    values.iter().sum::<u32>()
}
