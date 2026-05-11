int advance(int values[const 3]) {
    values = values + 1;
    return values[0];
}

int main(void) {
    int values[3] = {1, 2, 3};
    return advance(values);
}
