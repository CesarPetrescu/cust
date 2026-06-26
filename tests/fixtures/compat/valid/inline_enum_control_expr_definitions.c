int pick_with_if(int flag) {
    if ((enum IfInline { IF_VALUE = 5 })flag) {
        return IF_VALUE;
    }
    return 6;
}

int sum_with_loop(void) {
    int total = 0;
    int i = 0;
    while ((enum WhileInline { WHILE_LIMIT = 3 })i < WHILE_LIMIT) {
        total = total + WHILE_LIMIT;
        i = i + 1;
    }
    for ((void)(enum ForInitInline { FOR_INIT = 2 })0;
         (enum ForCondInline { FOR_LIMIT = 2 })i < 5;
         (void)(enum ForIncInline { FOR_STEP = 4 })0, i = i + FOR_STEP - 3) {
        total = total + FOR_INIT + FOR_LIMIT;
    }
    return total;
}

int choose_with_switch(void) {
    switch ((enum SwitchInline { SWITCH_CASE = 22 })SWITCH_CASE) {
    case SWITCH_CASE:
        return SWITCH_CASE;
    default:
        return 0;
    }
}

int main(void) {
    return pick_with_if(1) + sum_with_loop() + choose_with_switch();
}
