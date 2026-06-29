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

    if (sizeof(union IfChoice { int value; char tag; }) == sizeof(union IfChoice)) {
        union IfChoice choice = {3};
        total += choice.value;
    }

    while (total < 23 && sizeof(union WhileChoice { int step; char tag; }) == sizeof(union WhileChoice)) {
        union WhileChoice choice = {2};
        total += choice.step;
    }

    switch (sizeof(union SwitchChoice { int value; char tag; }) == sizeof(union SwitchChoice)) {
        case 1: {
            union SwitchChoice choice = {6};
            total += choice.value;
            break;
        }
        default:
            total += 40;
    }

    return total;
}
