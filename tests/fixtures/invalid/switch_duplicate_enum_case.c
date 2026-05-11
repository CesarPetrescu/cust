int main(void) {
    enum Codes {
        DUP_LEFT = 3,
        DUP_RIGHT = 3
    };

    switch (DUP_LEFT) {
    case DUP_LEFT:
        return 1;
    case DUP_RIGHT:
        return 2;
    default:
        return 3;
    }
}
