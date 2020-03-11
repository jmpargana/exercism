export class Matrix {
  constructor(matrix) {
    this.matrix = matrix
      .split('\n')
      .map(row => row
        .split(' ')
        .map(entry => +entry));
  }

  get rows() {
    return this.matrix
  }

  get columns() {
    return this.matrix[0]
      .map((_, index) => this.matrix
        .map(row => row[index]))
  }
}
