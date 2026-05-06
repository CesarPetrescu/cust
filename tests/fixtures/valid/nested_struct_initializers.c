struct Point {
    int x;
    char y;
};

struct Rect {
    struct Point origin;
    int width;
};

static struct Rect saved = {{4, 5}, 6};
const struct Rect config = {{7, 8}, 9};

int sum_rect(struct Rect rect) {
    return rect.origin.x + rect.origin.y + rect.width;
}

int main() {
    struct Rect local = {{1, 2}, 3};
    struct Rect partial = {{10}};

    local.origin.x += saved.origin.x;
    partial.origin.y = config.origin.y;
    partial.width = config.width;

    return sum_rect(local) + sum_rect(saved) + sum_rect(config) + sum_rect(partial) + sizeof(local.origin);
}
