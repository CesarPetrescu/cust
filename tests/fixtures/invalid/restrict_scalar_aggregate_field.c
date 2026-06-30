struct Bad {
    restrict int value;
};

int main(void) {
    struct Bad bad = {1};
    return bad.value;
}
