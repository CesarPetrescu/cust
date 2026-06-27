struct Packet {
    int values[2];
};

int main(void) {
    struct Packet left = {{1, 2}};
    struct Packet right = {{3, 4}};
    return right.values - left.values;
}
