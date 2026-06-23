int main(void) {
    int total = 0;

    total += sizeof(struct { int x; char tag; });
    total += sizeof(union { int value; char tag; });
    total += sizeof(const struct { char tag; int values[2]; });
    total += sizeof(struct { int x; } *);
    total += sizeof(struct { char c; }[3]);
    total += _Alignof(struct { char c; int x; });
    total += _Alignof(union { int x; char c; });
    total += _Alignof(struct { char text[4]; });

    return total;
}
