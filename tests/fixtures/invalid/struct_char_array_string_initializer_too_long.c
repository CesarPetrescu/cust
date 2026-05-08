struct Label {
    char text[3];
};

int main(void) {
    struct Label label = {"toolong"};
    return label.text[0];
}
