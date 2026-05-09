struct Point {
    int x;
    char tag;
};

int side_effect(void) {
    return 99;
}

int main(void) {
    int before = 3;
    int size = sizeof((char[]){'c', 'a', 't', 0});
    size += sizeof((char[5]){"dog"});
    size += sizeof((int[3]){1, before = side_effect(), 3});
    size += sizeof((struct Point[2]){{1, 'a'}, {2, 'b'}});
    return size + before;
}
