long int strtol(const char *text, char **end, int base);
long long int strtoll(const char *text, char **end, int base);
unsigned long int strtoul(const char *text, char **end, int base);
unsigned long long int strtoull(const char *text, char **end, int base);

int main(void) {
    char decimal[] = "  -123tail";
    char hexadecimal[] = "0x2A!";
    char octal[] = "0779";
    char binary[] = "10102";
    char base36[] = "zZ?";
    char no_digits[] = "  +x";
    char prefix_without_digits[] = "0xg";
    char *end = 0;

    if (strtol(decimal, &end, 10) != -123 || *end != 't') {
        return 1;
    }
    if (strtoul(hexadecimal, &end, 0) != 42 || *end != '!') {
        return 2;
    }
    if (strtol(octal, &end, 0) != 63 || *end != '9') {
        return 3;
    }
    if (strtoul(binary, &end, 2) != 10 || *end != '2') {
        return 4;
    }
    if (strtol(base36, &end, 36) != 1295 || *end != '?') {
        return 5;
    }
    if (strtol(no_digits, &end, 10) != 0 || end != no_digits) {
        return 6;
    }
    if (strtol(prefix_without_digits, &end, 0) != 0 || *end != 'x') {
        return 7;
    }
    if (strtoul("-7", 0, 10) != (unsigned long int)-7) {
        return 8;
    }
    if (strtoll("-2a!", &end, 16) != -42 || *end != '!') {
        return 9;
    }
    if (strtoull("377?", &end, 8) != 255 || *end != '?') {
        return 10;
    }
    if (sizeof(strtol(decimal, &end, 10)) != sizeof(long int) || *end != '?') {
        return 11;
    }
    return 0;
}
