#define BAD(left, right) left ## right
int main(void) { return BAD(+, *); }
