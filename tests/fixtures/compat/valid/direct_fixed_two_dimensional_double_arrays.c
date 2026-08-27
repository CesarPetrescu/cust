double global_matrix[2][3] = {{1.25, 2.0, 3.5}, {4.0}};
static double file_static_matrix[1][2] = {{5.5, 6.5}};

int touch_static(void) {
    static double local_static_matrix[1][2] = {{1.0, 2.0}};
    local_static_matrix[0][1] += 0.5;
    return local_static_matrix[0][1] == 2.5;
}

int main(void) {
    const double fixed[2][2] = {{1.5}, {2.5, 3.5}};
    double values[2][3] = {{0.5, 1.5}, {2.5}};
    int row = 0;
    int column = 0;
    double assigned = (values[row++][column++] = 4.0);
    double compounded = (values[1][0] += 0.75);
    double post = values[0][1]++;
    double pre = --values[1][0];

    values[1][2] = 7.25;
    global_matrix[1][1] = 4.5;
    file_static_matrix[0][0] *= 2.0;

    return assigned == 4.0
                && compounded == 3.25
                && post == 1.5
                && values[0][1] == 2.5
                && pre == 2.25
                && values[1][0] == 2.25
                && values[0][2] == 0.0
                && values[1][1] == 0.0
                && values[1][2] == 7.25
                && row == 1
                && column == 1
                && fixed[0][0] == 1.5
                && fixed[0][1] == 0.0
                && fixed[1][1] == 3.5
                && global_matrix[0][2] == 3.5
                && global_matrix[1][1] == 4.5
                && global_matrix[1][2] == 0.0
                && file_static_matrix[0][0] == 11.0
                && touch_static()
                && _Generic(values[0][0], double: 1, default: 0)
                && sizeof(values) == 6 * sizeof(double)
                && sizeof(values[0]) == 3 * sizeof(double)
                && sizeof(values[0][0]) == sizeof(double)
                && sizeof(double[2][3]) == 6 * sizeof(double)
                && _Alignof(double[2][3]) == _Alignof(double)
           ? 0
           : 1;
}
