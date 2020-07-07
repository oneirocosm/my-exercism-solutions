// pascals-triangle.js

export const rows = n => {
  const triangle = [];
  for (let i = 0; i < n; ++i) {
    const row = [];
    if (i == 0) {
      row.push(1);
    } else {
      row.push(1);
      const old_row = triangle[triangle.length - 1];
      const old_pairs = windowSlices(old_row, 2);
      for (const pair of old_pairs) {
        const sum = pair.reduce((a, b) => a + b, 0);
        row.push(sum);
      }
      row.push(1);
    }
    triangle.push(row);
  }
  return triangle;
};

const windowSlices = (fullArray, size) => {
  let i = 0;
  const slices = [];
  while (i + size <= fullArray.length) {
    slices.push(fullArray.slice(i, i + size));
    ++i;
  }
  return slices;
};
