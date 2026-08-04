char global_text[] = "wxyz";
char *global_slot = 0, *global_second_slot = 0;
char **global_output = &global_slot, **global_second_output = &global_second_slot;
static char *file_slot = 0;
static char **file_output = &file_slot;

static void set_output(char **output, char *value) {
    if (output) {
        *output = value;
    }
}

static void redirect_output(char **output, char **replacement, char *value) {
    output = replacement;
    *output = value;
}

static int update_static(char *value) {
    static char *local_slot = 0;
    static char **local_output = &local_slot;
    set_output(local_output, value + 2);
    return *local_slot;
}

int main(void) {
    char local_text[] = "abcd";
    char *local_slot = 0, *local_second_slot = 0, *mixed_slot = 0, *redirect_slot = 0,
         **mixed_output = &mixed_slot;
    char **local_output = &local_slot, **local_second_output = &local_second_slot;
    char **alias = local_output;
    char **null_output = 0;
    int markers = 0;
    char **selected = (markers++, 1 ? local_output : local_second_output);

    set_output(0 ? local_second_output : selected, local_text + 1);
    set_output(global_output, global_text + 1);
    *global_second_output = global_text + 2;
    set_output((markers++, local_second_output), local_text + 2);
    *mixed_output = local_text + 3;
    *file_output = global_text + 3;
    set_output(null_output, local_text);

    local_output = 0;
    if (local_output != 0) {
        return 5;
    }
    local_output = alias;
    local_output = (markers++, &local_slot);
    redirect_output(local_output, &redirect_slot, local_text + 3);

    if (markers != 3 || alias != selected || local_output != &local_slot ||
        *local_slot != 'b' ||
        *local_second_slot != 'c' || *mixed_slot != 'd' || *redirect_slot != 'd' ||
        *global_slot != 'x' || *global_second_slot != 'y' || *file_slot != 'z') {
        return 1;
    }
    if (null_output != 0 || null_output || sizeof(local_output) != sizeof(char *)) {
        return 2;
    }
    if (local_output != &local_slot || &local_slot != local_output ||
        local_output == &local_second_slot || &local_second_slot == local_output) {
        return 3;
    }
    return update_static(local_text) == 'c' ? 0 : 4;
}
