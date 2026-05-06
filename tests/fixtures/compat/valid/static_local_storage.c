int next_value() {
    static int counter = 10;
    counter += 1;
    return counter;
}

int table_sum(int index) {
    static int values[3];
    values[index] += index + 1;
    return values[0] + values[1] + values[2];
}

struct Point {
    int x;
    int y;
};

int move_point() {
    static struct Point point;
    point.x += 2;
    point.y += 3;
    return point.x + point.y;
}

int block_scoped_counter() {
    int result = 0;
    {
        static int hidden = 4;
        hidden += 2;
        result = hidden;
    }
    {
        static int hidden = 100;
        hidden += 1;
        result += hidden;
    }
    return result;
}

int main() {
    int a = next_value();
    int b = next_value();
    int persisted_array = table_sum(0) + table_sum(1) + table_sum(2) + table_sum(1);
    int persisted_struct = move_point() + move_point();
    int scoped = block_scoped_counter() + block_scoped_counter();

    return a + b + persisted_array + persisted_struct + scoped - 200;
}
