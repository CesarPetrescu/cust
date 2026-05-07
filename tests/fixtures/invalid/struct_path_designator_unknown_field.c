struct Inner {
    int x;
};

struct Packet {
    struct Inner inner;
};

int main(void) {
    struct Packet packet = {.inner.missing = 1};
    return packet.inner.x;
}
