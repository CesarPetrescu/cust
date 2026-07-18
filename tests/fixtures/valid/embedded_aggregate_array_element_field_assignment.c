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
    struct Box box = {{{3, 'a'}, {5, 'b'}}};
    struct Point points[2] = {{7, 'c'}, {9, 'd'}};
    struct Index index = {1};
    int direct_index_marker = 0;
    int direct_rhs_marker = 0;
    int reverse_pointer_marker = 0;
    int reverse_rhs_marker = 0;
    int ok = 0;

    int direct_assign_result = (box.points[0].value = 6);
    char direct_tag_assign_result = (box.points[0].tag = 'c');
    int reverse_assign_result = (index.value[points].value = 8);
    char reverse_tag_assign_result = (index.value[points].tag = 'f');

    int direct_compound_result =
        (box.points[direct_index_marker++].value += (direct_rhs_marker += 1, 4));
    char direct_char_compound_result = (box.points[1].tag += 2);
    int reverse_compound_result =
        (index.value[(reverse_pointer_marker += 1, points)].value *=
         (reverse_rhs_marker += 1, 2));
    char reverse_char_compound_result = (index.value[points].tag -= 1);

    ok += direct_assign_result == 6;
    ok += direct_tag_assign_result == 'c';
    ok += reverse_assign_result == 8;
    ok += reverse_tag_assign_result == 'f';
    ok += direct_compound_result == 10;
    ok += direct_char_compound_result == 'd';
    ok += reverse_compound_result == 16;
    ok += reverse_char_compound_result == 'e';
    ok += box.points[0].value == 10;
    ok += box.points[0].tag == 'c';
    ok += box.points[1].tag == 'd';
    ok += points[1].value == 16 && points[1].tag == 'e';
    ok += direct_index_marker == 1;
    ok += direct_rhs_marker == 1;
    ok += reverse_pointer_marker == 1;
    ok += reverse_rhs_marker == 1;

    return ok;
}
