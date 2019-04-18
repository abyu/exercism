use std::collections::HashMap;

pub fn raindrops(n: u32) -> String {
    let all_factors = get_factors(n);

    let value = all_factors
        .iter()
        .fold("".to_string(), |agg, val| agg + &translate(*val));

    if value.is_empty() {
        return n.to_string();
    }

    value
}

fn get_factors(n: u32) -> Vec<u32> {
    let all_numbers = 2..=n;

    all_numbers.filter(|&x| n % x == 0).collect::<Vec<u32>>()
}

fn translate(n: u32) -> String {
    let lang: HashMap<u32, String> = [
        (3, String::from("Pling")),
        (5, String::from("Plang")),
        (7, String::from("Plong")),
    ]
    .iter()
    .cloned()
    .collect();

    match lang.get(&n) {
        None => "".to_string(),
        Some(x) => x.to_string(),
    }
}
