int main(void) {
    int total = 0;

    if (sizeof(struct IfBox { int x; }) == sizeof(struct IfBox)) {
        struct IfBox box = {7};
        total += box.x;
    }

    while (total < 10 && sizeof(struct WhileCell { int step; }) == sizeof(struct WhileCell)) {
        struct WhileCell cell = {2};
        total += cell.step;
    }

    switch (sizeof(struct SwitchBox { int value; }) == sizeof(struct SwitchBox)) {
        case 1: {
            struct SwitchBox sw = {5};
            total += sw.value;
            break;
        }
        default:
            total += 40;
    }

    return total;
}
