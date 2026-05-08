struct Point {
    int x;
};

struct Size {
    int width;
};

struct Holder {
    struct Point *point;
};

int main(void) {
    struct Size size = {2};
    struct Holder holder;
    holder.point = &size;
    return 0;
}
