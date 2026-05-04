int main() {
    int *p = 0;
    {
        int x = 7;
        p = &x;
    }
    return *p;
}
