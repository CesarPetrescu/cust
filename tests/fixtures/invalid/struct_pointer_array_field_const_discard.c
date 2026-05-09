struct Packet {
    int values[2];
};

void mutate(int values[]) {
    values[0] = 9;
}

int main(void) {
    struct Packet packet = {{1, 2}};
    const struct Packet *slot = &packet;
    mutate(slot->values);
    return packet.values[0];
}
