#define VALUE 0x2aUL
#define STR(value) # value
#define XSTR(value) STR(value)
#define VSTR(...) #__VA_ARGS__
#define ID(value) value
#define L BAD
#define DIGRAPH_STR(value) %: value
#define EMPTY_MACRO
#define WRAP(value) left value+right

int text_equal(char *left, char *right) {
    int index = 0;
    while (left[index] == right[index] && left[index] != '\0') {
        index++;
    }
    return left[index] == right[index];
}

int main(void) {
    char *direct = STR(VALUE);
    char *expanded = XSTR(VALUE);
    char *nested = STR(ID(VALUE));
    char *spaced = STR(alpha /* normalized comment */ +
                       beta);
    char *literal = STR("a\\b" '\n');
    char *variadic = VSTR(left,  "q\\r");
    char *empty = STR();
    char *hash = STR(#);
    char *float_token = STR(1e+2);
    char *wide = XSTR(L"wide");
    char *large_escape = STR("\xFFFFFFFFFFFFFFFFFFFFFFFF");
    char *digraph = DIGRAPH_STR(token);
    char *digraph_argument = STR(%:);
    char *empty_expansion_gap = XSTR(left EMPTY_MACRO+right);
    char *empty_substitution_gap = XSTR(WRAP(EMPTY_MACRO));
    char *trimmed_substitution = XSTR(z+ID( a));
    char *trimmed_empty_substitution = XSTR(z+ID( EMPTY_MACRO)+a);

    return text_equal(direct, "VALUE")
            && text_equal(expanded, "0x2aUL")
            && text_equal(nested, "ID(VALUE)")
            && text_equal(spaced, "alpha + beta")
            && text_equal(literal, "\"a\\\\b\" '\\n'")
            && text_equal(variadic, "left, \"q\\\\r\"")
            && text_equal(empty, "")
            && text_equal(hash, "#")
            && text_equal(float_token, "1e+2")
            && text_equal(wide, "L\"wide\"")
            && text_equal(large_escape, "\"\\xFFFFFFFFFFFFFFFFFFFFFFFF\"")
            && text_equal(digraph, "token")
            && text_equal(digraph_argument, "%:")
            && text_equal(empty_expansion_gap, "left +right")
            && text_equal(empty_substitution_gap, "left +right")
            && text_equal(trimmed_substitution, "z+a")
            && text_equal(trimmed_empty_substitution, "z++a")
        ? 0
        : 1;
}
