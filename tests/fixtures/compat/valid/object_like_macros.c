#define EMPTY// trailing comment
#define INTEGER int
#define BASE_COUNT 4
#define COUNT BASE_COUNT
#define EXTRA 2
#define LIMIT (COUNT + EXTRA)
#define VALUE (COUNT * EXTRA)

INTEGER values[LIMIT] = {[COUNT] = VALUE};

enum Bounds {
    LAST = LIMIT - 1
};

int main(void) {
    // COUNT remains ordinary comment text.
    char *name = "COUNT";
    char initial = 'C';
    int runtime = VALUE + LAST;

    return EMPTY values[4] == 8 && runtime == 13 && name[0] == initial ? 0 : 1;
}
