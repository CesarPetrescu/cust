struct Packet {
    int values[4];
};

struct Holder {
    struct Packet packet;
};

int score_packet(int *start, int *middle, int *last) {
    int score = 0;
    if (start < middle) {
        score += 1;
    }
    if (middle <= last) {
        score += 2;
    }
    if (last > middle) {
        score += 4;
    }
    if (last >= start) {
        score += 8;
    }
    return score + (last - start) * 3;
}

int main(void) {
    struct Packet packet = {{1, 2, 3, 4}};
    struct Holder holder = {{{5, 6, 7, 8}}};
    struct Packet packets[1] = {{{9, 10, 11, 12}}};
    struct Packet *slot = &packet;
    int *middle = packet.values + 1;
    int direct = (packet.values < &packet.values[3])
        + ((packet.values + 2) > middle) * 2
        + ((&packet.values[3] - packet.values) == 3) * 4
        + (middle ? 8 : 0);
    int nested = score_packet(holder.packet.values, holder.packet.values + 1, &holder.packet.values[3]);
    int element = packets[0].values < &packets[0].values[2] ? 16 : 0;
    int arrow = slot->values < &slot->values[2] ? 32 : 0;
    return direct + nested + element + arrow;
}
