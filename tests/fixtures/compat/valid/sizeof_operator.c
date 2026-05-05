int main() {
    char bytes[5];
    return sizeof(char) + sizeof("abc") + sizeof bytes;
}
