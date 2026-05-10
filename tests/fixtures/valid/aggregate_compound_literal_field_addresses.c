struct Point { int x; int y; };
struct Box { struct Point inner; int tail; };
union Number { int value; char tag; };

int bump(int *slot, int amount) {
    *slot += amount;
    return *slot;
}

int main(void) {
    int *px = &((struct Point){4, 8}).x;
    int before = *px;
    int after = bump(px, 5);

    int *nested = &((struct Box){{2, 3}, 4}).inner.y;
    int nested_before = *nested;
    *nested = *nested + 6;

    int *uv = &((union Number){7}).value;
    *uv += 1;

    return before + after + nested_before + *nested + *uv;
}
