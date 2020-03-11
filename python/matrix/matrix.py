class Matrix:
    def __init__(self, matrix_string):
        matrix = [row.split() for row in matrix_string.split('\n')]

        assert all(map(lambda x: len(x[0]) == len(x[1]),
                       zip(matrix[:-1], matrix[1:])))

        self.matrix = matrix

    def row(self, index):
        return [int(i) for i in self.matrix[index - 1]]
        
    def column(self, index):
        return [int(i) for i in [col[index - 1] for col in self.matrix]]
