int main(void) {
    int size = sizeof((char[]){'c', 'a', 't', 0});
    size += sizeof((char[5]){"dog"});
    return size;
}
