int double_value(int value);
void store_total(int *slot, int value);
int first_char(char *text);

int main() {
    int result = double_value(6);
    int total = 0;
    store_total(&total, result + first_char("abc"));
    return total - 'a';
}

int double_value(int value) {
    return value * 2;
}

void store_total(int *slot, int value) {
    *slot = value;
}

int first_char(char *text) {
    return text[0];
}
