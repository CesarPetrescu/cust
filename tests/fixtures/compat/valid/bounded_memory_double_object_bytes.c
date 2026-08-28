void *memcpy(void *, const void *, unsigned long int);
void *memmove(void *, const void *, unsigned long int);
int memcmp(const void *, const void *, unsigned long int);
void *memset(void *, int, unsigned long int);
void *memchr(const void *, int, unsigned long int);

typedef double Real;

int main(void) {
    const Real source = 6.25;
    Real destination = -1.0;
    if (memcpy(&destination, &source, sizeof(source)) != &destination) return 1;
    if (destination != source) return 2;
    if (memcmp(&destination, &source, sizeof(source)) != 0) return 3;

    double values[4] = {1.5, 2.5, 3.5, 4.5};
    if (memmove(values + 1, values, sizeof(double) * 3) != values + 1) return 4;
    if (values[0] != 1.5 || values[1] != 1.5 || values[2] != 2.5 || values[3] != 3.5) return 5;

    double zero = 9.5;
    if (memset(&zero, 0, sizeof(zero)) != &zero) return 6;
    if (zero != 0.0) return 7;
    double *found = memchr(&zero, 0, sizeof(zero));
    if (found != &zero) return 8;
    if (memchr(&source, 255, 0) != 0) return 9;
    return 0;
}