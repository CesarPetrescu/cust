typedef int Count;
typedef char Byte;

int bump(int value) {
    return (int)(value + 1);
}

int main(void) {
    int x = (int)3;
    char c = (char)(x + 2);
    Count count = (Count)(c + 4);
    Byte byte = (Byte)count;
    int grouped = ((int)(byte + 1));
    int side_effect = 0;

    if ((char)(side_effect = 1)) {
        side_effect = side_effect + 2;
    }

    return bump(grouped) + side_effect;
}
