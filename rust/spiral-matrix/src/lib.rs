pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    }

    let mut matrix = Vec::new();
    for _ in 0..size {
        matrix.push(vec![0; size as usize]);
    }

    update_matrix(&mut matrix, 1, 0, size as usize);
    matrix
}

fn update_matrix(matrix: &mut [Vec<u32>], start: u32, i: usize, size: usize) {
    let left_col_ind = i;
    let right_col_ind = (i + size).saturating_sub(1);
    let top_row_ind = i;
    let bottom_row_ind = (i + size).saturating_sub(1);

    println!("{:?}", matrix);

    let size_val = size as u32;
    for j in (0..size).rev() {
        let j_val = j as u32;
        matrix[bottom_row_ind - j][left_col_ind] = start + 3 * size_val - 3 + j_val;
        matrix[bottom_row_ind][right_col_ind - j] = start + 2 * size_val - 2 + j_val;
        matrix[top_row_ind + j][right_col_ind] = start + size_val - 1 + j_val;
        matrix[top_row_ind][left_col_ind + j] = start + j_val;
    }

    let inner_size = size.saturating_sub(2);
    if inner_size != 0 {
        update_matrix(matrix, start + 4 * size_val - 4, i + 1, inner_size);
    }
}
