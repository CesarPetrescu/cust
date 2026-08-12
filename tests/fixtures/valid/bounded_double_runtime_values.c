double global_value = 1.25;
double global_zero;
static double file_saved = 2.5;
enum { CASTED_FROM_DOUBLE = (int)1.5 };
enum { BOOL_FROM_DOUBLE = (_Bool)0.5 };

int main(void) {
    static double block_zero;
    static double block_saved = -2.;
    double local;
    double scientific = 1e1 + 1.25e-1 - .125;
    double arithmetic;
    double forwarded;
    double comma_value;
    double mutation = 1.5;
    double old;
    _Alignas(8) double aligned = 1.5;
    double loop_total = 0.0;

    local = +global_value + .75;
    arithmetic = (local * 3.0 - block_saved) / 2.0;
    forwarded = 1 ? arithmetic : local;
    comma_value = (local = 2.5, local);
    mutation += 0.5;
    mutation *= 3.0;
    mutation -= 2.0;
    mutation /= 2.0;
    old = mutation++;
    --mutation;
    for (double step = 1.5; step < 2.5; step += 0.5) {
        loop_total += step;
    }

    return global_zero == 0.0
                && block_zero == 0.0
                && file_saved == 2.5
                && scientific == 10.0
                && arithmetic == 4.0
                && -block_saved == 2.0
                && forwarded == arithmetic
                && comma_value == local
                && local != arithmetic
                && local < arithmetic
                && local <= arithmetic
                && arithmetic > local
                && arithmetic >= local
                && mutation == 2.0
                && old == 2.0
                && aligned == 1.5
                && loop_total == 3.5
                && CASTED_FROM_DOUBLE == 1
                && BOOL_FROM_DOUBLE == 1
                && _Generic(1.0 + 2.0, double: 1, default: 0)
                && arithmetic && !0.0
                && (0.0 || arithmetic)
                && (int)3.75 == 3
                && (double)3 == 3.0
                && sizeof(double) == 8
                && sizeof(const double) == 8
                && sizeof(global_value) == 8
                && _Alignof(double) == 8
           ? 0
           : 1;
}
