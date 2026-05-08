struct Label {
    char text[5];
    int value;
};

struct Packet {
    struct Label label;
    char code[4];
};

int sum_text(char text[5]) {
    return text[0] + text[1] + text[2] + text[3] + text[4];
}

int main(void) {
    struct Label first = {"cat", 3};
    struct Label second = {.value = 4, .text = "hi"};
    struct Packet nested = {{"dog", 5}, "xy"};
    struct Packet designated = {.label.text = "A\x2a", .label.value = 6, .code = "Q"};
    struct Label labels[2] = {{"one", 1}, [1] = {.text = "two", .value = 2}};

    int total = 0;
    total += first.text[0] == 'c';
    total += first.text[3] == 0;
    total += first.value == 3;
    total += second.text[0] == 'h';
    total += second.text[2] == 0;
    total += nested.label.text[1] == 'o';
    total += nested.code[2] == 0;
    total += designated.label.text[1] == 42;
    total += designated.code[0] == 'Q';
    total += labels[0].text[2] == 'e';
    total += labels[1].text[0] == 't';

    return total;
}
