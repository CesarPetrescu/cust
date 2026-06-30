struct Point {
    int x;
    char tag;
};

int main(void) {
    int values[4] = {3, 5, 7, 11};
    struct Point points[2] = {{13, 2}, {17, 4}};
    int total = 0;
    int one = 1;

    if (sizeof(*(values + (sizeof(struct SizePtrOffset { int value; }) == sizeof(struct SizePtrOffset)))) == sizeof(int)) {
        total += values[one];
    }
    if (sizeof(*(values + (sizeof(union SizePtrUnion { int value; char tag; }) == sizeof(union SizePtrUnion)) + one)) == sizeof(values[0])) {
        total += values[2];
    }
    if (sizeof(*(points + (sizeof(struct SizeAggPtr { char tag; }) == sizeof(struct SizeAggPtr)))) == sizeof(struct Point)) {
        total += points[1].x;
    }
    if (sizeof(*(&((struct SizeFieldPtr { int value; }){19}).value)) == sizeof(int)) {
        total += ((struct SizeReadBack { int value; }){23}).value;
    }

    return total;
}
