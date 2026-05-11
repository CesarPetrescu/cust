struct Point {
    int x;
    int y;
};

union Number {
    int value;
    char tag;
};

enum Status { READY = 1 };

extern int total;
extern char marker;
extern int values[3];
extern struct Point origin;
extern struct Point points[2];
extern union Number number;
extern enum Status status;
extern int *cursor;

int total = 4;
char marker = 'A';
int values[3] = {2, 3, 4};
struct Point origin = {5, 6};
struct Point points[2] = {{1, 2}, {3, 4}};
union Number number = {2};
enum Status status = READY;
int *cursor = values;

int main(void) {
    return total + (marker == 'A') + cursor[1] + values[2] + origin.y + points[1].x + number.value + status;
}
