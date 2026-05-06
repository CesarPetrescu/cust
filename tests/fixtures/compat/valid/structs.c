struct Point {
    int x;
    char y;
};

struct Point global;

int main() {
    struct Point local;
    global.x = 8;
    global.y = 2;
    local.x = 5;
    local.y = 4;
    return global.x + global.y + local.x + local.y;
}
