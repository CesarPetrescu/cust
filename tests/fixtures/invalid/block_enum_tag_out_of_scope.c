int main() {
    {
        enum Local { LOCAL = 1 };
    }
    typedef enum Local Local;
    Local value = LOCAL;
    return value;
}
