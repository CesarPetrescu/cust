struct Pair {
    int x;
    int y;
};

int take_pair(int *slot, struct Pair pair) {
    *slot = *slot + pair.x + pair.y;
    return *slot;
}

int main(void) {
    int values[4] = {3, 5, 7, 9};
    int total = 0;
    int *cursor = values;
    struct Pair point = {1, 2};

    (cursor = values + (sizeof(enum PtrExpr { PTR_EXPR_OFFSET = 2 }) == sizeof(enum PtrExpr) ? PTR_EXPR_OFFSET : 0));
    total += *cursor + PTR_EXPR_OFFSET;

    (point = (struct Pair){
        .x = _Alignof(enum AggExpr { AGG_EXPR_VALUE = 4 }) ? AGG_EXPR_VALUE : 0,
        .y = (enum AggCast { AGG_CAST_VALUE = 6 })0 + AGG_CAST_VALUE,
    });
    total += point.x + point.y + AGG_EXPR_VALUE + AGG_CAST_VALUE;

    total += take_pair(cursor, (struct Pair){
        sizeof(enum CallAgg { CALL_AGG_VALUE = 8 }) ? CALL_AGG_VALUE : 0,
        CALL_AGG_VALUE + 1,
    });

    return total + values[2] + PTR_EXPR_OFFSET + AGG_EXPR_VALUE + AGG_CAST_VALUE + CALL_AGG_VALUE;
}
