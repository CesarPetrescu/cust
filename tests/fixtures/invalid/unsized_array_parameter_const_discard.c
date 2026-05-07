int mutate(int values[]) {
    values[0] = 3;
    return values[0];
}

int main(void) {
    const int values[1] = {1};
    return mutate(values);
}
