struct Pair {
    char tag;
    int value;
};

union Number {
    char tag;
    int value;
};

int main(void) {
    int score = 0;
    score = score + (_Alignof(char) == 1);
    score = score + (_Alignof(char[4]) == _Alignof(char));
    score = score + (_Alignof(struct Pair) >= _Alignof(char));
    score = score + (_Alignof(struct Pair) >= _Alignof(int));
    score = score + (_Alignof(union Number) >= _Alignof(char));
    score = score + (_Alignof(union Number) >= _Alignof(int));
    return score;
}
