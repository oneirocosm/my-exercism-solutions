export class GradeSchool {
  constructor() {
    this.db = {};
  }
  roster() {
    return JSON.parse(JSON.stringify(this.db));
  }

  add(student, grade) {
    if (this.db[grade]) {
      this.db[grade].push(student);
      this.db[grade].sort();
    } else {
      this.db[grade] = [student];
    }
  }

  grade(num) {
    const students = this.db[num] || [];
    return [...students];
  }
}
