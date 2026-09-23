double read(const double (*rows)[2]) { return rows[1][0]; }

double (*advance(double (*rows)[2]))[2] { return rows + 1; }

int main(void) {
    double values[2][2] = {{1.25, 2.5}, {3.75, 4.5}};
    int marker = 0;
    if (sizeof(advance(values)[marker++]) != 2 * sizeof(double)) return 1;
    if (marker != 0) return 2;
    advance(values)[0][1] = 5.25;
    return (int)(read(values) * 4) + (int)(values[1][1] * 4);
}
