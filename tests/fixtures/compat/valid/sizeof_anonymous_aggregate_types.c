#include <stdalign.h>

int main(void) {
    int total = 0;

    total += sizeof(struct { char c; }) == sizeof(char);
    total += sizeof(struct { int x; }) == sizeof(int);
    total += sizeof(union { int x; char c; }) == sizeof(int);
    total += sizeof(struct { int values[2]; }) == sizeof(int[2]);
    total += sizeof(struct { int x; } *) == sizeof(void *);
    total += _Alignof(struct { char c; }) == alignof(char);
    total += _Alignof(union { int x; char c; }) == alignof(int);

    return total;
}
