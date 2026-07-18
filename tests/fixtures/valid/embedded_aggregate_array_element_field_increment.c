struct Point {
    int value;
    char tag;
};

struct Box {
    struct Point points[2];
};

struct Index {
    int value;
};

int main(void) {
    struct Box box = {{{10, 'a'}, {20, 'b'}}};
    struct Point points[2] = {{30, 'c'}, {40, 'd'}};
    struct Index index = {1};
    int marker = 0;
    int ok = 0;

    int direct_prefix = ++box.points[marker++].value;
    int direct_postfix = box.points[marker].value--;
    char direct_char_old = box.points[0].tag++;
    char direct_char_new = --box.points[0].tag;

    int reverse_prefix = ++index.value[points].value;
    int reverse_postfix = index.value[points].value--;
    char reverse_char_old = index.value[points].tag++;
    char reverse_char_new = --index.value[points].tag;
    int reverse_once = index.value[(marker += 1, points)].value++;

    ok += direct_prefix == 11;
    ok += direct_postfix == 20;
    ok += box.points[0].value == 11;
    ok += box.points[1].value == 19;
    ok += direct_char_old == 'a';
    ok += direct_char_new == 'a';
    ok += reverse_prefix == 41;
    ok += reverse_postfix == 41;
    ok += reverse_char_old == 'd';
    ok += reverse_char_new == 'd';
    ok += reverse_once == 40;
    ok += points[1].value == 41;
    ok += points[1].tag == 'd';
    ok += marker == 2;

    return ok;
}
