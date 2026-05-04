int main() {
    int x = 1;
    int y = 0;
    {
        int x = 40;
        y = x + 2;
    }
    return y + x;
}
