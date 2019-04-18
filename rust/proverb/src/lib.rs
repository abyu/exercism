pub fn build_proverb(list: &[&str]) -> String {
    let end = format!("And all for the want of a {}.", list[0]);

    let verses = list
        .iter()
        .zip(list.iter().skip(1))
        .map(|(x, y)| format!("For want of a {} the {} was lost.", x, y))
        .collect::<Vec<String>>()
        .join("\n");

    return format!("{}\n{}", verses, end);
}
