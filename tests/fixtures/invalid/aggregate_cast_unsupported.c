struct Point { int x; };

int main(void) {
    struct Point p = {1};
    return (struct Point)p;
}
