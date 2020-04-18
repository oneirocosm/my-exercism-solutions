use itertools::Itertools;

pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }

    let mut proverb = String::new();
    for (the_want, the_lost) in list.iter().tuple_windows() {
        proverb.push_str(&format!(
            "For want of a {} the {} was lost.\n",
            the_want, the_lost,
        ));
    }
    proverb.push_str(&format!("And all for the want of a {}.", list[0]));

    proverb
}
