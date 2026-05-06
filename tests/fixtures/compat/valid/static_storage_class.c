static int total = 3;
static const int limit = 6;
static char marker = 'A';
static int values[3];

struct Point {
    int x;
};

static struct Point global_point;
static int *slot = &total;

static int bump(int amount);

static void seed() {
    values[0] = 4;
    global_point.x = 5;
    *slot += 1;
}

static int bump(int amount) {
    total += amount;
    return total;
}

int main() {
    seed();
    return bump(2) + values[0] + global_point.x + (marker == 'A') + limit;
}
