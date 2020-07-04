// matrix.js

export class Matrix {
  constructor(data_text) {
    const rows_text = data_text.split("\n");
    let rows = [];
    for (const row_text of rows_text) {
      rows.push(Matrix.parse_into_row(row_text));
    }
    this._rows = rows;

    const num_rows = rows.length;
    this._num_rows = rows.length;
    this._num_cols = this.calc_col_length();
  }

  static parse_into_row(row_text) {
    return row_text.split(" ").map(Number);
  }

  calc_col_length() {
    let num_cols;
    if (this._num_rows == 0) {
      num_cols = 0;
    } else {
      num_cols = this._rows[0].length;
      if (!this._rows.every(row => row.length == num_cols)) {
        throw "All rows must be the same length";
      }
    }
    return num_cols;
  }

  get rows() {
    return this._rows;
  }

  get columns() {
    let cols = [];
    for (let i = 0; i < this._num_cols; i++) {
      let col = [];
      for (let j = 0; j < this._num_rows; j++) {
        col.push(this._rows[j][i]);
      }
      cols.push(col);
    }

    return cols;
  }
}
