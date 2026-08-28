typedef double Matrix[2][3];
typedef Matrix MatrixAlias;
typedef const double ConstMatrix[2][2];
typedef const Matrix ConstMatrixAlias;

Matrix global_matrix = {{1.25, 2.0, 3.5}, {4.0}};
static Matrix file_static_matrix = {{5.5, 6.5}};

int touch_static(void) {
    static Matrix local_static_matrix = {{1.0, 2.0}};
    local_static_matrix[0][1] += 0.5;
    return local_static_matrix[0][1] == 2.5;
}

int main(void) {
    Matrix first = {{0.5, 1.5}, {2.5}};
    MatrixAlias second = {{3.0}, {4.0, 5.0, 6.0}}, third = {{7.0}};
    ConstMatrix fixed = {{1.5}, {2.5, 3.5}};
    ConstMatrixAlias chained_fixed = {{8.5}, {9.5, 10.5}};
    int row = 0;
    int column = 0;
    double assigned = (first[row++][column++] = 4.0);
    double compounded = (first[1][0] += 0.75);
    double post = first[0][1]++;
    double pre = --first[1][0];

    first[1][2] = 7.25;
    global_matrix[1][1] = 4.5;
    file_static_matrix[0][0] *= 2.0;
    second[0][2] = 8.0;
    third[1][2] += 9.0;

    return assigned == 4.0
                && compounded == 3.25
                && post == 1.5
                && first[0][1] == 2.5
                && pre == 2.25
                && first[1][0] == 2.25
                && first[0][2] == 0.0
                && first[1][1] == 0.0
                && first[1][2] == 7.25
                && row == 1
                && column == 1
                && fixed[0][1] == 0.0
                && fixed[1][1] == 3.5
                && chained_fixed[0][0] == 8.5
                && chained_fixed[0][1] == 0.0
                && chained_fixed[1][2] == 0.0
                && global_matrix[0][2] == 3.5
                && global_matrix[1][1] == 4.5
                && global_matrix[1][2] == 0.0
                && file_static_matrix[0][0] == 11.0
                && second[0][2] == 8.0
                && second[1][2] == 6.0
                && third[0][0] == 7.0
                && third[1][2] == 9.0
                && touch_static()
                && _Generic(first[0][0], double: 1, default: 0)
                && sizeof(Matrix) == 6 * sizeof(double)
                && sizeof(first[0]) == 3 * sizeof(double)
                && sizeof(first[0][0]) == sizeof(double)
                && sizeof(MatrixAlias) == sizeof(Matrix)
                && _Alignof(Matrix) == _Alignof(double)
           ? 0
           : 1;
}
