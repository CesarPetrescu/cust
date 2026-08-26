typedef const double ConstRow[2];

int main(void) {
    const double *pointer = (ConstRow){1.0, 2.0};
    pointer[0] = 3.0;
    return 0;
}
