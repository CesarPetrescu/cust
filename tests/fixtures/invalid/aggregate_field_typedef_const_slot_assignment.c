typedef int * const ConstIntSlot;

struct Cursor {
    ConstIntSlot fixed, backup;
};

int main(void) {
    int first[1] = {1};
    int second[1] = {2};
    struct Cursor cursor = {first, first};
    cursor.fixed = second;
    return *cursor.fixed;
}
