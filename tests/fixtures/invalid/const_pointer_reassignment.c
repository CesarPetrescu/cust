int main() {
    int left = 1;
    int right = 2;
    int * const p = &left;
    p = &right;
    return *p;
}
