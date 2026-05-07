int main(void) {
    typedef struct Pair {
        int x;
        int y;
    } Pair;
    Pair p = {3, 4};

    int total = p.x * 10 + p.y;
    {
        typedef union Number {
            int value;
            char tag;
        } Number;
        Number n = {5};
        total = total + n.value;
    }

    return total;
}
