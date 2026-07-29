#define NAME VALUE
#define DIRECT_NAME 7
#define DIRECT_VALUE 9
#define RESULT_1 5
#define CAT(left, right) left ## right
#define XCAT(left, right) CAT(left, right)
#define CAT3(first, second, third) first ## second ## third
#define VCAT(prefix, ...) prefix ## __VA_ARGS__
#define DIGRAPH_CAT(left, right) left %:%: right
#define PLUS_ASSIGN + ## =
#define STR(value) #value
#define XSTR(value) STR(value)
#define PASTE_STR(left, right) XSTR(left ## right)

#if CAT(RESULT_, 1) == 5
#define CONDITION_SCORE 11
#else
#define CONDITION_SCORE 0
#endif

int pasted_keyword(void) {
    CAT(ret, urn) 6;
}

int main(void) {
    int variable = 13;
    int direct = CAT(DIRECT_, NAME);
    int expanded = XCAT(DIRECT_, NAME);
    int chained = CAT3(DIRECT_, NA, ME);
    int variadic = VCAT(var, iable);
    int digraph = DIGRAPH_CAT(DIRECT_, NAME);
    int number = CAT(12, 3);
    int multiple_tokens = 1;
    CAT(multiple_tokens +, = 2);
    int adjusted = 1;
    adjusted PLUS_ASSIGN 4;
    int digraph_values[2] = {17, 19};
    int digraph_block = 0;
    CAT(<, %) digraph_block += 23; CAT(%, >)
    char *pasted_text = PASTE_STR(DIRECT_, NAME);
    char *pasted_hash = XSTR(CAT(%, :));
    char *pasted_digraph_operator = XSTR(CAT(%:, %:));
    char *pasted_operator = XSTR(CAT(#, #));

    return direct == 7
            && expanded == 9
            && chained == 7
            && variadic == 13
            && digraph == 7
            && number == 123
            && multiple_tokens == 3
            && adjusted == 5
            && digraph_values CAT(<, :) 1 CAT(:, >) == 19
            && digraph_block == 23
            && CAT(, variable) == 13
            && CAT(variable, ) == 13
            && (1 CAT(, ) + 2) == 3
            && CAT3(, var, iable) == 13
            && CAT3(var, iable, ) == 13
            && pasted_text[0] == '7'
            && pasted_text[1] == '\0'
            && pasted_hash[0] == '%' && pasted_hash[1] == ':'
            && pasted_hash[2] == '\0'
            && pasted_digraph_operator[0] == '%'
            && pasted_digraph_operator[1] == ':'
            && pasted_digraph_operator[2] == '%'
            && pasted_digraph_operator[3] == ':'
            && pasted_digraph_operator[4] == '\0'
            && pasted_operator[0] == '#' && pasted_operator[1] == '#'
            && pasted_operator[2] == '\0'
            && pasted_keyword() == 6
            && CONDITION_SCORE == 11
        ? 0
        : 1;
}
