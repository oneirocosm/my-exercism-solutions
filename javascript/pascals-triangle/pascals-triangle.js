export const rows = n => {
  if (n == 0) {
    return [];
  }

  const triangle = [[1]];
  for (let i = 1; i < n; ++i) {
    const row = [];
    const prev_row = triangle[i - 1];
    for (let j = 0; j < i + 1; ++j) {
      const prev_left = prev_row[j - 1] || 0;
      const prev_right = prev_row[j] || 0;
      row.push(prev_left + prev_right);
    }
    triangle.push(row);
  }
  return triangle;
};
