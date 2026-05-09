struct Packet {
    int values[3];
    char label[4];
};

int sum_values(int values[]) {
    values[1] += 5;
    return values[0] + values[1] + values[2];
}

int sum_label(char text[]) {
    text[2] = 'Z';
    return text[0] + text[1] + text[2] + text[3];
}

int main(void) {
    struct Packet packet = {{2, 3, 4}, "ab"};
    struct Packet *slot = &packet;
    int total = sum_values(slot->values);
    total += slot->values[1];
    int *middle = &slot->values[1];
    *middle += 7;
    total += slot->values[1];
    total += sum_label(slot->label);
    total += slot->label[2];
    char *tail = &slot->label[2];
    *tail = 'C';
    total += slot->label[2];
    return total - 256;
}
