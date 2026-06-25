int main(void) {
    {
        struct Hidden {
            int x;
        };
        struct Hidden value = {1};
        (void)value.x;
    }
    struct Hidden value = {2};
    return value.x;
}
