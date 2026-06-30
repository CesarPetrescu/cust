int main(void) {
    int marker = 0;
    int values[3] = {4, 5, 6};
    char text[3] = "hi";

    int char_ok = sizeof((marker = marker + 1, (char){7})) == sizeof(char);
    int pointer_ok = sizeof((marker = marker + 10, values + 1)) == sizeof(values + 1);
    int array_element_ok = sizeof((marker = marker + 100, text[1])) == sizeof(char);
    int scalar_ok = sizeof((marker = marker + 1000, values[0])) == sizeof(int);

    if (marker != 0) {
        return 200;
    }

    return char_ok + pointer_ok + array_element_ok + scalar_ok;
}
