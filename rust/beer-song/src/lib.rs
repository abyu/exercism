pub fn verse(n: i32) -> String {
    let how_many_beers = how_many(n);
    let how_many_left = how_many(n - 1);

    let what_next = match n - 1 {
        -1 => "Go to the store and buy some more",
        0 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    };

    return format!(
        "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
        how_many_beers,
        how_many_beers.to_lowercase(),
        what_next,
        how_many_left.to_lowercase()
    );
}

pub fn sing(start: i32, end: i32) -> String {
    let range = end..=start;
    let mut lyrics: Vec<String> = range.map(verse).collect();
    lyrics.reverse();

	lyrics.join("\n")
}

pub fn how_many(n: i32) -> String {
    match n {
        -1 => String::from("99 bottles"),
        0 => String::from("No more bottles"),
        1 => String::from("1 bottle"),
        _ => format!("{} bottles", n),
    }
}
