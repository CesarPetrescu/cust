int global_a = 2, global_b, global_c = 5;
const int global_const_a = 7, global_const_b = 11;
char global_ch = 'A', global_next = 'B';

int main(void) {
    int local_a = 3, local_b, local_c = local_a + global_a;
    char local_ch = 'a', local_next = local_ch + 1;
    _Bool truth = local_c == 5, falsehood;
    static int seen = 4, bonus = 6;
    int total = global_a + global_b + global_c;
    total = total + seen + bonus;
    total = total + global_const_a + global_const_b;
    total = total + (global_next - global_ch);
    total = total + local_a + local_b + local_c;
    total = total + (local_next - local_ch);
    total = total + truth + falsehood;
    for (int i = 0, j = 3; i < 3; i = i + 1) {
        total = total + j;
        j = j - 1;
    }
    return total;
}
