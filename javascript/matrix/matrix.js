// matrix.js

export class Matrix {
  constructor(data_text) {
    const rows_text = data_text.split("\n");
    const rows = rows_text.map(row_text => row_text.split(" ").map(Number));
    this._rows = rows;

    this._num_rows = rows.length;
    this._num_cols = this.calc_num_cols();
  }

  static parse_into_row(row_text) {
    return row_text.split(" ").map(Number);
  }

  calc_num_cols() {
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
    return [...Array(this._num_cols).keys()].map(i =>
      [...Array(this._num_rows).keys()].map(j => this._rows[j][i])
    );
  }
}
