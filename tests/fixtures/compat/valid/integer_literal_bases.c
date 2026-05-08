int main(void) {
    int hex = 0x2a;
    int upper_hex = 0X10;
    int octal = 052;
    int mixed = hex + upper_hex + octal;
    int from_array[3] = {0x1, 02, 0X3};
    return mixed + from_array[0] + from_array[1] + from_array[2];
}
