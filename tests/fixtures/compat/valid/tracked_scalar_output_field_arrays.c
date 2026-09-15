/* Native compilers are external oracles; compare identities and same-type sizes. */

typedef char *character_ptr;
typedef character_ptr *character_output;
struct character_box { char **outputs[3]; character_output extra[1]; };
struct character_outer { struct character_box box; };
static char character_value = 65;
static char *character_slot = &character_value;
static struct character_box character_global = { { &character_slot }, {0} };
static int character_calls;
static char **character_forward(char **out) { character_calls++; return out; }
static void character_set(char **out, char *value) { *out = value; }
static int check_character(void) {
    char value = 65; char *first = &value, *second = 0;
    struct character_box direct = { .outputs[2] = &first, .extra = { &second } };
    struct character_outer objects[1] = { { { { &first, 0, &second }, {0} } } };
    struct character_outer *p = objects;
    const struct character_outer *read_only = p;
    static struct character_box persistent = { { &character_slot }, {0} };
    int index = 0;
    objects[index++].box.outputs[1] = character_forward(direct.outputs[2]);
    character_set(p->box.outputs[2], &value);
    **read_only->box.outputs[1] = 66;
    persistent.outputs[1] = character_global.outputs[0];
    struct character_box copy = objects[0].box;
    direct = copy;
    copy.outputs[0] = 0;
    if (sizeof(p->box.outputs[0]) != sizeof(&first)) return 0;
    if (sizeof(p->box.outputs) != 3 * sizeof(p->box.outputs[0])) return 0;
    if (sizeof(**p->box.outputs[0]) != sizeof(value)) return 0;
    return index == 1 && character_calls == 1 && value == 66 && second == first
        && direct.outputs[0] == &first && copy.outputs[0] == 0
        && persistent.outputs[1] == &character_slot;
}

typedef int *integer_ptr;
typedef integer_ptr *integer_output;
struct integer_box { int **outputs[3]; integer_output extra[1]; };
struct integer_outer { struct integer_box box; };
static int integer_value = 7;
static int *integer_slot = &integer_value;
static struct integer_box integer_global = { { &integer_slot }, {0} };
static int integer_calls;
static int **integer_forward(int **out) { integer_calls++; return out; }
static void integer_set(int **out, int *value) { *out = value; }
static int check_integer(void) {
    int value = 7; int *first = &value, *second = 0;
    struct integer_box direct = { .outputs[2] = &first, .extra = { &second } };
    struct integer_outer objects[1] = { { { { &first, 0, &second }, {0} } } };
    struct integer_outer *p = objects;
    const struct integer_outer *read_only = p;
    static struct integer_box persistent = { { &integer_slot }, {0} };
    int index = 0;
    objects[index++].box.outputs[1] = integer_forward(direct.outputs[2]);
    integer_set(p->box.outputs[2], &value);
    **read_only->box.outputs[1] = 9;
    persistent.outputs[1] = integer_global.outputs[0];
    struct integer_box copy = objects[0].box;
    direct = copy;
    copy.outputs[0] = 0;
    if (sizeof(p->box.outputs[0]) != sizeof(&first)) return 0;
    if (sizeof(p->box.outputs) != 3 * sizeof(p->box.outputs[0])) return 0;
    if (sizeof(**p->box.outputs[0]) != sizeof(value)) return 0;
    return index == 1 && integer_calls == 1 && value == 9 && second == first
        && direct.outputs[0] == &first && copy.outputs[0] == 0
        && persistent.outputs[1] == &integer_slot;
}

typedef _Bool *boolean_ptr;
typedef boolean_ptr *boolean_output;
struct boolean_box { _Bool **outputs[3]; boolean_output extra[1]; };
struct boolean_outer { struct boolean_box box; };
static _Bool boolean_value = 0;
static _Bool *boolean_slot = &boolean_value;
static struct boolean_box boolean_global = { { &boolean_slot }, {0} };
static int boolean_calls;
static _Bool **boolean_forward(_Bool **out) { boolean_calls++; return out; }
static void boolean_set(_Bool **out, _Bool *value) { *out = value; }
static int check_boolean(void) {
    _Bool value = 0; _Bool *first = &value, *second = 0;
    struct boolean_box direct = { .outputs[2] = &first, .extra = { &second } };
    struct boolean_outer objects[1] = { { { { &first, 0, &second }, {0} } } };
    struct boolean_outer *p = objects;
    const struct boolean_outer *read_only = p;
    static struct boolean_box persistent = { { &boolean_slot }, {0} };
    int index = 0;
    objects[index++].box.outputs[1] = boolean_forward(direct.outputs[2]);
    boolean_set(p->box.outputs[2], &value);
    **read_only->box.outputs[1] = 1;
    persistent.outputs[1] = boolean_global.outputs[0];
    struct boolean_box copy = objects[0].box;
    direct = copy;
    copy.outputs[0] = 0;
    if (sizeof(p->box.outputs[0]) != sizeof(&first)) return 0;
    if (sizeof(p->box.outputs) != 3 * sizeof(p->box.outputs[0])) return 0;
    if (sizeof(**p->box.outputs[0]) != sizeof(value)) return 0;
    return index == 1 && boolean_calls == 1 && value == 1 && second == first
        && direct.outputs[0] == &first && copy.outputs[0] == 0
        && persistent.outputs[1] == &boolean_slot;
}

typedef double *floating_ptr;
typedef floating_ptr *floating_output;
struct floating_box { double **outputs[3]; floating_output extra[1]; };
struct floating_outer { struct floating_box box; };
static double floating_value = 1.25;
static double *floating_slot = &floating_value;
static struct floating_box floating_global = { { &floating_slot }, {0} };
static int floating_calls;
static double **floating_forward(double **out) { floating_calls++; return out; }
static void floating_set(double **out, double *value) { *out = value; }
static int check_floating(void) {
    double value = 1.25; double *first = &value, *second = 0;
    struct floating_box direct = { .outputs[2] = &first, .extra = { &second } };
    struct floating_outer objects[1] = { { { { &first, 0, &second }, {0} } } };
    struct floating_outer *p = objects;
    const struct floating_outer *read_only = p;
    static struct floating_box persistent = { { &floating_slot }, {0} };
    int index = 0;
    objects[index++].box.outputs[1] = floating_forward(direct.outputs[2]);
    floating_set(p->box.outputs[2], &value);
    **read_only->box.outputs[1] = 2.75;
    persistent.outputs[1] = floating_global.outputs[0];
    struct floating_box copy = objects[0].box;
    direct = copy;
    copy.outputs[0] = 0;
    if (sizeof(p->box.outputs[0]) != sizeof(&first)) return 0;
    if (sizeof(p->box.outputs) != 3 * sizeof(p->box.outputs[0])) return 0;
    if (sizeof(**p->box.outputs[0]) != sizeof(value)) return 0;
    return index == 1 && floating_calls == 1 && value == 2.75 && second == first
        && direct.outputs[0] == &first && copy.outputs[0] == 0
        && persistent.outputs[1] == &floating_slot;
}

int main(void) { return check_character() && check_integer() && check_boolean() && check_floating() ? 0 : 1; }
