export class List {
  constructor(values = []) {
    this.values = values;
  }

  append(other) {
    let base_length = this.length();
    let additional_length = other.length();
    let length = base_length + additional_length;
    let new_values = new Array(length);

    for (let i = 0; i < base_length; i++) {
      new_values[i] = this.values[i];
    }
    for (let i = 0; i < additional_length; i++) {
      new_values[base_length + i] = other.values[i];
    }

    return new List(new_values);
  }

  concat(others) {
    let concatenated = new List(this.values);
    for (let other of others.values) {
      concatenated = concatenated.append(other);
    }

    return concatenated;
  }

  filter(bool_func) {
    let filtered = new List();
    for (let value of this.values) {
      if (bool_func(value)) {
        filtered = filtered.append(new List([value]));
      }
    }

    return filtered;
  }

  map(map_function) {
    let mapped = new List();
    for (let value of this.values) {
      let mapped_value = map_function(value);
      mapped = mapped.append(new List([mapped_value]));
    }

    return mapped;
  }

  length() {
    let i = 0;
    while (this.values[i]) {
      i++;
    }
    return i;
  }

  foldl(folder, initial_value) {
    let acc = initial_value;

    for (let value of this.values) {
      acc = folder(acc, value);
    }

    return acc;
  }

  foldr(folder, initial_value) {
    return this.reverse().foldl(folder, initial_value);
  }

  reverse() {
    let length = this.length();
    let reversed = new Array(length);

    for (let i = 0; i < length; i++) {
      reversed[i] = this.values[length - i - 1];
    }

    return new List(reversed);
  }
}
