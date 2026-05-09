union Counter {
    int value;
    char tag;
};

int next_value(int delta) {
    static union Counter saved = {1};
    saved.value = saved.value + delta;
    return saved.value;
}

int bump_tag(void) {
    static union Counter marker;
    marker.tag = marker.tag + 2;
    return marker.tag;
}

int main(void) {
    int first = next_value(4);
    int second = next_value(5);
    int tag_first = bump_tag();
    int tag_second = bump_tag();
    return first + second + tag_first + tag_second;
}
