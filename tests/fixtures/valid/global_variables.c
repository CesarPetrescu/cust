int counter = 2;
char marker = 'A';
int values[4];
int *chosen = 0;

void bump(int amount) {
    counter += amount;
    values[0] = counter;
}

int read_global_array(int index) {
    return values[index];
}

int main() {
    values[1] = 3;
    bump(5);
    chosen = &values[1];
    *chosen += marker - 'A' + 4;
    return counter + read_global_array(0) + values[1] + (chosen == &values[1]);
}
