struct Point {
    int x;
    int y;
};

int add(int (value), int *(cursor), struct Point (point)) {
    int (local) = value + cursor[1];
    struct Point (copy) = point;
    int (items)[3] = {local, copy.x, copy.y};
    int (total) = items[0] + items[1] + items[2];
    return total;
}

int main(void) {
    int (values)[3] = {2, 5, 9};
    int *(cursor) = values;
    struct Point (point) = {7, 11};
    int (result) = add(values[0], cursor, point);
    return result;
}
