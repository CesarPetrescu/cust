struct Packet {
    int values[2];
};

int main(void) {
    struct Packet packet = {.values[2] = 1};
    return packet.values[0];
}
