use itertools::Itertools;

struct VerseData {
    what: String,
    loc: String,
    act: String,
    left: String,
}

impl VerseData {
    fn to_string(&self) -> String {
        format!(
            "{what} {loc}, {what2}.\n{act}, {left} {loc}.\n",
            what = self.what,
            what2 = self.what.to_lowercase(),
            loc = self.loc,
            act = self.act,
            left = self.left
        )
    }

    fn from(n: i32) -> VerseData {
        let what = match n {
            0 => "No more bottles of beer".to_string(),
            1 => "1 bottle of beer".to_string(),
            _ => format!("{} bottles of beer", n),
        };

        let left = match n {
            0 => "99 bottles of beer".to_string(),
            1 => "no more bottles of beer".to_string(),
            2 => "1 bottle of beer".to_string(),
            _ => format!("{} bottles of beer", n - 1),
        };

        let act = match n {
            0 => "Go to the store and buy some more".to_string(),
            1 => "Take it down and pass it around".to_string(),
            _ => "Take one down and pass it around".to_string(),
        };

        VerseData {
            what,
            loc: "on the wall".to_string(),
            left,
            act,
        }
    }
}

pub fn verse(n: i32) -> String {
    VerseData::from(n).to_string()
}

pub fn sing(start: i32, end: i32) -> String {
    (end..=start).rev().map(verse).join("\n")
}
