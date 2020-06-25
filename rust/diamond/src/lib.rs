pub fn get_diamond(c: char) -> Vec<String> {
    let max_num = number_of_alpha(c);

    add_inner_rows(Vec::new(), 0, max_num)
}

fn add_inner_rows(mut rows: Vec<String>, row_num: u8, max_row: u8) -> Vec<String> {
    let outer = (max_row - row_num) as usize;
    let new_row: String = if row_num == 0 {
        format!(
            "{sp:^outer$}{c}{sp:^outer$}",
            sp = "",
            c = alpha_of_number(row_num),
            outer = outer
        )
    } else {
        let inner = (2 * max_row as usize) - 2 * outer - 1;
        format!(
            "{sp:^outer$}{c}{sp:^inner$}{c}{sp:^outer$}",
            sp = "",
            c = alpha_of_number(row_num),
            outer = outer,
            inner = inner
        )
    };

    if row_num == max_row {
        rows.push(new_row);
    } else {
        rows.push(new_row.clone());
        rows = add_inner_rows(rows, row_num + 1, max_row);
        rows.push(new_row);
    }

    rows
}

fn number_of_alpha(c: char) -> u8 {
    return (c as u8) - ('A' as u8);
}

fn alpha_of_number(n: u8) -> char {
    return (n + ('A' as u8)) as char;
}
