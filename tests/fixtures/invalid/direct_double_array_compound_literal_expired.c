double *escape(void) {
    return (double[]){1.0, 2.0};
}

int main(void) {
    double *pointer = escape();
    return pointer[0] == 1.0;
}
