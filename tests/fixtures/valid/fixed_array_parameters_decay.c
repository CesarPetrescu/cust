int sum_tail(int values[3]) {
    values[3] = values[3] + 1;
    return values[0] + values[3];
}

int pointer_size(int values[3]) {
    return sizeof(values);
}

int char_tail(char text[2]) {
    return text[3];
}

int main(void) {
    int values[4] = {1, 2, 3, 4};
    int total = sum_tail(values);
    total = total + values[3];
    total = total + pointer_size(values);
    return total + char_tail("abc");
}
