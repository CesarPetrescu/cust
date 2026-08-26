int main(void) {
    double *pointer = (const double[]){1.0};
    return pointer[0] == 1.0;
}
