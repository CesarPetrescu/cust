int main(void) {
    typedef const int Scores[2];
    int *scores = (Scores){1, 2};
    return scores[0];
}
