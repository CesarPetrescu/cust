int main(void) {
    int scalar_fields = ((struct { int x; int y; }){.y = 5, .x = 2}).x
        + ((union { int value; char tag; }){7}).value;
    int array_fields = ((struct { int x; int y; }[]){{1, 2}, {.x = 8, .y = 9}})[1].y
        + ((union { int value; char tag; }[]){{3}, {.tag = 4}})[0].value;
    return scalar_fields + array_fields;
}
