int leaf(void) {
    const char *name = __func__;
    return sizeof(__func__) == sizeof("leaf")
        && name[0] == 'l'
        && __func__[3] == 'f'
        && __func__[4] == 0;
}

const char *recur(int depth) {
    if (sizeof(__func__) != sizeof("recur")
        || __func__[0] != 'r'
        || __func__[4] != 'r') {
        return 0;
    }
    if (depth == 0) {
        return __func__;
    }
    const char *nested = recur(depth - 1);
    return nested == __func__ ? nested : 0;
}

int wrapper(void) {
    const char *recursive_name = recur(3);
    return sizeof(__func__) == sizeof("wrapper")
        && __func__[0] == 'w'
        && leaf()
        && recursive_name[4] == 'r';
}

int main(void) {
    if (sizeof(__func__) != sizeof("main")
        || __func__[0] != 'm'
        || __func__[3] != 'n'
        || __func__[4] != 0) {
        return 1;
    }
    return wrapper() ? 73 : 2;
}