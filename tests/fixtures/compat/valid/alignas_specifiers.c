_Alignas(8) int global_count = 3;

struct Cell {
    _Alignas(int) char tag;
    _Alignas(16) int value;
};

int read_cell(struct Cell cell) {
    return cell.tag + cell.value;
}

int main(void) {
    _Alignas(8) int local = 4;
    static _Alignas(8) int saved = 5;
    struct Cell cell = {1, 2};
    for (_Alignas(8) int i = 0; i < 2; i = i + 1) {
        local = local + i;
    }
    return global_count + local + saved + read_cell(cell);
}
