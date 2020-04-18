use itertools::Itertools;

pub fn sing(start: i32, end: i32) -> String {
    (end..=start).rev().map(verse).join("\n")
}

pub fn verse(number_of_bottles: i32) -> String {
    match number_of_bottles {
        0 => "No more bottles of beer on the wall, \
            no more bottles of beer.\n\
            Go to the store and buy some more, \
            99 bottles of beer on the wall.\n"
            .into(),
        1 => "1 bottle of beer on the wall, \
            1 bottle of beer.\n\
            Take it down and pass it around, \
            no more bottles of beer on the wall.\n"
            .into(),
        2 => "2 bottles of beer on the wall, \
            2 bottles of beer.\n\
            Take one down and pass it around, \
            1 bottle of beer on the wall.\n"
            .into(),
        n => format!(
            "{num_before} bottles of beer on the wall, \
            {num_before} bottles of beer.\n\
            Take one down and pass it around, \
            {num_after} bottles of beer on the wall.\n",
            num_before = n,
            num_after = n - 1,
        ),
    }
}
