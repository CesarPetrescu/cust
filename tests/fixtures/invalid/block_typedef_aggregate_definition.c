int main(void) {
    typedef struct Local {
        int value;
    } Local;
    Local local = {1};
    return local.value;
}
