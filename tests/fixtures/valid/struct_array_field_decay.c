struct Packet {
    int values[3];
    char tag[4];
};

int sum_values(int values[]) {
    values[1] += 5;
    return values[0] + values[1] + values[2];
}

int bump_first(int *values) {
    int *slot = &values[0];
    *slot += 2;
    return values[0];
}

int first_char(char text[]) {
    text[1] = text[1] + 1;
    return text[0] + text[1] + text[2];
}

int main(void) {
    struct Packet packet = {{1, 2, 3}, "\001\002"};
    struct Packet packets[2] = {{{4, 5, 6}, "bc"}, {{7, 8, 9}, "de"}};

    int total = sum_values(packet.values);
    total += packet.values[1];

    int *middle = &packet.values[1];
    *middle += 4;
    total += packet.values[1];

    total += bump_first(packets[1].values);
    int *tail = &packets[1].values[2];
    *tail += 1;
    total += packets[1].values[2];

    total += first_char(packet.tag);
    total += packet.tag[1];

    return total;
}
