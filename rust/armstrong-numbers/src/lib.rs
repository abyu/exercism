pub fn is_armstrong_number(num: u32) -> bool {
    let str_value = num.to_string();
    let exp = str_value.len() as u32;

    let fval = str_value
        .chars()
        .map(|val| val.to_digit(10).unwrap().pow(exp))
        .sum::<u32>();

    num == fval
}
