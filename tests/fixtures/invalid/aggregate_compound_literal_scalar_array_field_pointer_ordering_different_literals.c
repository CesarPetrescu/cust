struct Packet {
    int values[2];
};

int main(void) {
    return ((struct Packet){{1, 2}}).values < ((struct Packet){{3, 4}}).values;
}
