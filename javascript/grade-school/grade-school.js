export class GradeSchool {
  constructor() {
    this.grades = {};
  }

  roster() {
    return Object.keys(this.grades).reduce(
      (acc, gradeNum) => ({ ...acc, [gradeNum]: this.grade(gradeNum) }),
      {}
    );
  }

  add(student, gradeNum) {
    this.grades[gradeNum] = (this.grades[gradeNum] || []).concat(student).sort();
  }

  grade(gradeNum) {
    return [...(this.grades[gradeNum] || [])];
  }
}
