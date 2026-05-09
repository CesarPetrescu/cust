struct Packet {
    int values[3];
};

int mutate_int(int *slot) {
    *slot = *slot + 4;
    return *slot;
}

int main(void) {
    int total = (((struct Packet){{1, 2, 3}}).values[0] = 7);
    total = total + (((struct Packet){{3, 4, 5}}).values[1] += 6);
    total = total + ++((struct Packet){{5, 6, 7}}).values[2];
    total = total + ((struct Packet){{8, 9, 10}}).values[0]++;
    total = total + mutate_int(&((struct Packet){{1, 2, 3}}).values[1]);
    return total;
}
