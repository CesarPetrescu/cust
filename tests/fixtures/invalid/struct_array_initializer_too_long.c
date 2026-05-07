struct Packet {
    int values[2];
};

int main(void) {
    struct Packet packet = {{1, 2, 3}};
    return packet.values[0];
}
