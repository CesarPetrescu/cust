struct Packet {
    int values[3];
    char tag[2];
};

int sum_packet(struct Packet packet) {
    packet.values[1] += 4;
    packet.tag[0]++;
    return packet.values[0] + packet.values[1] + packet.values[2] + packet.tag[0] + packet.tag[1];
}

int main(void) {
    struct Packet packet = {{2, 3, 0}, {'A', 1}};
    packet.values[2] = 5;
    packet.values[0]++;
    packet.tag[1] += 2;

    struct Packet copy;
    copy = packet;
    copy.values[1] = copy.values[1] + 7;

    return packet.values[0]
        + packet.values[1]
        + packet.values[2]
        + packet.tag[0]
        + packet.tag[1]
        + copy.values[1]
        + sum_packet(packet);
}
