struct Packet {
    const int values[2];
};

int main(void) {
    int *slot = &((struct Packet){{1, 2}}).values[1];
    return *slot;
}
