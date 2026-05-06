typedef int Word;

int main() {
    {
        typedef char Local;
        Local value = 1;
    }

    Local leaked = 2;
    return leaked;
}
