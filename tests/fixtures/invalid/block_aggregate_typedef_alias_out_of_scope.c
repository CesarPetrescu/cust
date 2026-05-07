int main(void) {
    {
        typedef struct Hidden {
            int value;
        } Hidden;
        Hidden h = {1};
    }
    struct Hidden leaked;
    return 0;
}
