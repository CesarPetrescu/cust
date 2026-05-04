int overwrite(char text[3]) {
    text[0] = 'X';
    return text[0];
}

int main() {
    return overwrite("hi");
}
