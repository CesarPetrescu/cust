typedef const double ConstRow[2];

int main(void) {
    double *pointer = (ConstRow){1.0, 2.0};
    return pointer[0] == 1.0;
}
