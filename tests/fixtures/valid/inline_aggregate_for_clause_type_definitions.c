int main(void) {
    int total = 0;

    for (int i = sizeof(struct ForInitBox { int value; }) == sizeof(struct ForInitBox); i < 2; i++) {
        struct ForInitBox box = {3};
        total += box.value;
    }

    int j = 0;
    for (; j < 1 && sizeof(struct ForCondBox { int value; }) == sizeof(struct ForCondBox); j++) {
        struct ForCondBox box = {5};
        total += box.value;
    }

    int k = 0;
    for (; k < 1; k += sizeof(union ForIncChoice { int value; char tag; }) == sizeof(union ForIncChoice)) {
        union ForIncChoice choice = {7};
        total += choice.value;
    }

    return total;
}