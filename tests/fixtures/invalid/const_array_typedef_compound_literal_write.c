int main(void) {
    typedef const int Scores[2];
    const int *scores = (Scores){1, 2};
    scores[0] = 9;
    return scores[0];
}
