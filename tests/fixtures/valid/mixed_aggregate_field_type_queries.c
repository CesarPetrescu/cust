typedef int *IntPtr;
typedef const int *ConstView;
typedef int * const Slot;

struct Scene {
    struct Point {
        int x, y;
        int values[2], offsets[2];
    } start, end;
    struct {
        IntPtr head, tail;
        ConstView view;
        Slot fixed;
    } cursor;
};

int sum_scene(struct Scene *scene) {
    int total = 0;
    total += scene->start.x + scene->end.y;
    total += scene->start.values[1] + scene->end.offsets[0];
    total += *scene->cursor.head + *(scene->cursor.tail - 1);
    total += scene->cursor.view[1] + *scene->cursor.fixed;
    total += sizeof(scene->start.values) == 2 * sizeof(scene->start.values[0]);
    total += _Alignof(struct Point[2]) == _Alignof(struct Point);
    return total;
}

int main(void) {
    int values[3] = {5, 7, 11};
    struct Scene scene = {
        {1, 2, {3, 4}, {5, 6}},
        {.y = 8, .values = {9, 10}, .offsets = {11, 12}},
        {values + 1, values + 2, values, values},
    };
    return sum_scene(&scene);
}
