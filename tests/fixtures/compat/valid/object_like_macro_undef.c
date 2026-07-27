#define VALUE 3
#define ALIAS VALUE
#undef/* before name */ VALUE /* trailing comment */

int VALUE = 4;

int value_before_redefinition(void) {
    return ALIAS;
}

#undef UNKNOWN
#define VALUE 5

int main(void) {
    return value_before_redefinition() == 4 && VALUE == 5 && ALIAS == 5 ? 0 : 1;
}
