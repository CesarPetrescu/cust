void write_int(int *p) {
    *p = 200;
}

int main(void) {
    char c = 1;
    write_int(&c);
    return c;
}
