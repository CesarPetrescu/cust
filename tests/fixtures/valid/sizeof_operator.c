int main() {
    int values[3];
    char bytes[5];
    int x = 0;
    int *p = &x;
    char *text = "abc";

    return sizeof(int)
        + sizeof(char)
        + sizeof x
        + sizeof values
        + sizeof bytes
        + sizeof p
        + sizeof(&x)
        + sizeof("abc")
        + sizeof text
        + sizeof(values[0])
        + sizeof(bytes[0])
        + (sizeof(x = 99) == 8)
        + (x == 0);
}
