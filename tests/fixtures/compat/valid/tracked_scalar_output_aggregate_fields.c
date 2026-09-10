/* Portable identity and same-type size relationships; no host-address assumptions. */
static int forwards;

typedef char *character_pointer;
typedef character_pointer *character_output;
struct character_inner { char **direct; character_output alias; };
struct character_outer { struct character_inner inner; };
static char character_value = 65;
static char *character_slot = &character_value;
static struct character_outer character_global = { { &character_slot, &character_slot } };

static char **character_forward(char **output) { forwards++; return output; }
static void character_set(char **output, char *value) { *output = value; }
union character_union { char **a; char **b; };
struct character_embedded { struct character_inner a[1]; };
static struct character_inner character_box(char **output) {
    struct character_inner box = { output, output };
    forwards++;
    return box;
}
static int check_character(void) {
    forwards = 0;
    char value = 65;
    char *left = &value, *right = 0;
    struct character_outer boxes[2] = { { { &left, 0 } }, { { &right, 0 } } };
    struct character_outer *view = boxes;
    const struct character_outer *read_only = view;
    static struct character_outer persistent = { { &character_slot, &character_slot } };
    struct character_outer copy = { { 0, 0 } };
    int index = 0;
    boxes[index++].inner.alias = character_forward(view->inner.direct);
    character_set(read_only->inner.direct, &value);
    *boxes[1].inner.direct = &value;
    **view->inner.alias = 66;
    copy = boxes[0];
    if (sizeof(character_forward(boxes[index].inner.alias)) != sizeof(&left)) return 0;
    if (sizeof(*view->inner.direct) != sizeof(left)) return 0;
    if (sizeof(**view->inner.direct) != sizeof(value)) return 0;
    union character_union overlap = { &left };
    if (overlap.b != &left) return 0;
    overlap.b = &right;
    if (overlap.a != &right) return 0;
    union character_union union_copy = overlap;
    overlap.a = &left;
    if (union_copy.a != &right || union_copy.b != &right || overlap.b != &left) return 0;
    union_copy = overlap;
    if (union_copy.a != &left || union_copy.b != &left) return 0;
    struct character_embedded embedded = { { { &left, &right } } };
    if (sizeof(**embedded.a[0].direct) != sizeof(value)) return 0;
    if (sizeof(**((struct character_inner){ &left, &right }).direct) != sizeof(value)) return 0;
    if (sizeof(**character_box(&left).direct) != sizeof(value)) return 0;
    if (embedded.a[0].direct != &left) return 0;
    if (((struct character_inner){ &left, &right }).alias != &right) return 0;
    embedded.a[0].alias = &left;
    if (embedded.a[0].alias != embedded.a[0].direct || forwards != 1) return 0;
    if (character_box(&left).direct != &left || forwards != 2) return 0;
    return forwards == 2 && index == 1 && value == 66 && left == &value && right == &value
        && copy.inner.alias == &left && boxes[0].inner.direct != boxes[1].inner.direct
        && persistent.inner.direct == character_global.inner.alias;
}

typedef int *integer_pointer;
typedef integer_pointer *integer_output;
struct integer_inner { int **direct; integer_output alias; };
struct integer_outer { struct integer_inner inner; };
static int integer_value = 17;
static int *integer_slot = &integer_value;
static struct integer_outer integer_global = { { &integer_slot, &integer_slot } };

static int **integer_forward(int **output) { forwards++; return output; }
static void integer_set(int **output, int *value) { *output = value; }
union integer_union { int **a; int **b; };
struct integer_embedded { struct integer_inner a[1]; };
static struct integer_inner integer_box(int **output) {
    struct integer_inner box = { output, output };
    forwards++;
    return box;
}
static int check_integer(void) {
    forwards = 0;
    int value = 17;
    int *left = &value, *right = 0;
    struct integer_outer boxes[2] = { { { &left, 0 } }, { { &right, 0 } } };
    struct integer_outer *view = boxes;
    const struct integer_outer *read_only = view;
    static struct integer_outer persistent = { { &integer_slot, &integer_slot } };
    struct integer_outer copy = { { 0, 0 } };
    int index = 0;
    boxes[index++].inner.alias = integer_forward(view->inner.direct);
    integer_set(read_only->inner.direct, &value);
    *boxes[1].inner.direct = &value;
    **view->inner.alias = 23;
    copy = boxes[0];
    if (sizeof(integer_forward(boxes[index].inner.alias)) != sizeof(&left)) return 0;
    if (sizeof(*view->inner.direct) != sizeof(left)) return 0;
    if (sizeof(**view->inner.direct) != sizeof(value)) return 0;
    union integer_union overlap = { &left };
    if (overlap.b != &left) return 0;
    overlap.b = &right;
    if (overlap.a != &right) return 0;
    union integer_union union_copy = overlap;
    overlap.a = &left;
    if (union_copy.a != &right || union_copy.b != &right || overlap.b != &left) return 0;
    union_copy = overlap;
    if (union_copy.a != &left || union_copy.b != &left) return 0;
    struct integer_embedded embedded = { { { &left, &right } } };
    if (sizeof(**embedded.a[0].direct) != sizeof(value)) return 0;
    if (sizeof(**((struct integer_inner){ &left, &right }).direct) != sizeof(value)) return 0;
    if (sizeof(**integer_box(&left).direct) != sizeof(value)) return 0;
    if (embedded.a[0].direct != &left) return 0;
    if (((struct integer_inner){ &left, &right }).alias != &right) return 0;
    embedded.a[0].alias = &left;
    if (embedded.a[0].alias != embedded.a[0].direct || forwards != 1) return 0;
    if (integer_box(&left).direct != &left || forwards != 2) return 0;
    return forwards == 2 && index == 1 && value == 23 && left == &value && right == &value
        && copy.inner.alias == &left && boxes[0].inner.direct != boxes[1].inner.direct
        && persistent.inner.direct == integer_global.inner.alias;
}

typedef _Bool *boolean_pointer;
typedef boolean_pointer *boolean_output;
struct boolean_inner { _Bool **direct; boolean_output alias; };
struct boolean_outer { struct boolean_inner inner; };
static _Bool boolean_value = 0;
static _Bool *boolean_slot = &boolean_value;
static struct boolean_outer boolean_global = { { &boolean_slot, &boolean_slot } };

static _Bool **boolean_forward(_Bool **output) { forwards++; return output; }
static void boolean_set(_Bool **output, _Bool *value) { *output = value; }
union boolean_union { _Bool **a; _Bool **b; };
struct boolean_embedded { struct boolean_inner a[1]; };
static struct boolean_inner boolean_box(_Bool **output) {
    struct boolean_inner box = { output, output };
    forwards++;
    return box;
}
static int check_boolean(void) {
    forwards = 0;
    _Bool value = 0;
    _Bool *left = &value, *right = 0;
    struct boolean_outer boxes[2] = { { { &left, 0 } }, { { &right, 0 } } };
    struct boolean_outer *view = boxes;
    const struct boolean_outer *read_only = view;
    static struct boolean_outer persistent = { { &boolean_slot, &boolean_slot } };
    struct boolean_outer copy = { { 0, 0 } };
    int index = 0;
    boxes[index++].inner.alias = boolean_forward(view->inner.direct);
    boolean_set(read_only->inner.direct, &value);
    *boxes[1].inner.direct = &value;
    **view->inner.alias = 1;
    copy = boxes[0];
    if (sizeof(boolean_forward(boxes[index].inner.alias)) != sizeof(&left)) return 0;
    if (sizeof(*view->inner.direct) != sizeof(left)) return 0;
    if (sizeof(**view->inner.direct) != sizeof(value)) return 0;
    union boolean_union overlap = { &left };
    if (overlap.b != &left) return 0;
    overlap.b = &right;
    if (overlap.a != &right) return 0;
    union boolean_union union_copy = overlap;
    overlap.a = &left;
    if (union_copy.a != &right || union_copy.b != &right || overlap.b != &left) return 0;
    union_copy = overlap;
    if (union_copy.a != &left || union_copy.b != &left) return 0;
    struct boolean_embedded embedded = { { { &left, &right } } };
    if (sizeof(**embedded.a[0].direct) != sizeof(value)) return 0;
    if (sizeof(**((struct boolean_inner){ &left, &right }).direct) != sizeof(value)) return 0;
    if (sizeof(**boolean_box(&left).direct) != sizeof(value)) return 0;
    if (embedded.a[0].direct != &left) return 0;
    if (((struct boolean_inner){ &left, &right }).alias != &right) return 0;
    embedded.a[0].alias = &left;
    if (embedded.a[0].alias != embedded.a[0].direct || forwards != 1) return 0;
    if (boolean_box(&left).direct != &left || forwards != 2) return 0;
    return forwards == 2 && index == 1 && value == 1 && left == &value && right == &value
        && copy.inner.alias == &left && boxes[0].inner.direct != boxes[1].inner.direct
        && persistent.inner.direct == boolean_global.inner.alias;
}

typedef double *floating_pointer;
typedef floating_pointer *floating_output;
struct floating_inner { double **direct; floating_output alias; };
struct floating_outer { struct floating_inner inner; };
static double floating_value = 1.25;
static double *floating_slot = &floating_value;
static struct floating_outer floating_global = { { &floating_slot, &floating_slot } };

static double **floating_forward(double **output) { forwards++; return output; }
static void floating_set(double **output, double *value) { *output = value; }
union floating_union { double **a; double **b; };
struct floating_embedded { struct floating_inner a[1]; };
static struct floating_inner floating_box(double **output) {
    struct floating_inner box = { output, output };
    forwards++;
    return box;
}
static int check_floating(void) {
    forwards = 0;
    double value = 1.25;
    double *left = &value, *right = 0;
    struct floating_outer boxes[2] = { { { &left, 0 } }, { { &right, 0 } } };
    struct floating_outer *view = boxes;
    const struct floating_outer *read_only = view;
    static struct floating_outer persistent = { { &floating_slot, &floating_slot } };
    struct floating_outer copy = { { 0, 0 } };
    int index = 0;
    boxes[index++].inner.alias = floating_forward(view->inner.direct);
    floating_set(read_only->inner.direct, &value);
    *boxes[1].inner.direct = &value;
    **view->inner.alias = 2.75;
    copy = boxes[0];
    if (sizeof(floating_forward(boxes[index].inner.alias)) != sizeof(&left)) return 0;
    if (sizeof(*view->inner.direct) != sizeof(left)) return 0;
    if (sizeof(**view->inner.direct) != sizeof(value)) return 0;
    union floating_union overlap = { &left };
    if (overlap.b != &left) return 0;
    overlap.b = &right;
    if (overlap.a != &right) return 0;
    union floating_union union_copy = overlap;
    overlap.a = &left;
    if (union_copy.a != &right || union_copy.b != &right || overlap.b != &left) return 0;
    union_copy = overlap;
    if (union_copy.a != &left || union_copy.b != &left) return 0;
    struct floating_embedded embedded = { { { &left, &right } } };
    if (sizeof(**embedded.a[0].direct) != sizeof(value)) return 0;
    if (sizeof(**((struct floating_inner){ &left, &right }).direct) != sizeof(value)) return 0;
    if (sizeof(**floating_box(&left).direct) != sizeof(value)) return 0;
    if (embedded.a[0].direct != &left) return 0;
    if (((struct floating_inner){ &left, &right }).alias != &right) return 0;
    embedded.a[0].alias = &left;
    if (embedded.a[0].alias != embedded.a[0].direct || forwards != 1) return 0;
    if (floating_box(&left).direct != &left || forwards != 2) return 0;
    return forwards == 2 && index == 1 && value == 2.75 && left == &value && right == &value
        && copy.inner.alias == &left && boxes[0].inner.direct != boxes[1].inner.direct
        && persistent.inner.direct == floating_global.inner.alias;
}

struct metadata_box { int **p; };
struct metadata_holder { struct metadata_box box; };
static struct metadata_box *metadata_array = _Generic(0, int: (struct metadata_box[]){{0}});

static int check_metadata_portability(void) {
    static struct metadata_holder persistent = { { 0 } };
    struct metadata_box box = { 0 };
    int *slot = 0;
    int index = 0;
    (*(index++, &box)).p = &slot;
    struct rows_holder { int rows[2][3]; } h;
    struct rows_holder *p = &h;
    int rows[2][3];
    int (*row)[3] = rows;
    if (sizeof(_Generic(0, default: p->rows)) != 6 * sizeof(int)) return 0;
    if (sizeof(p->rows[0]) != 3 * sizeof(int)) return 0;
    if (sizeof(*(rows + 1)) != 3 * sizeof(int)) return 0;
    if (sizeof(*(row + 1)) != 3 * sizeof(int)) return 0;
    return box.p == &slot && index == 1 && persistent.box.p == 0 && metadata_array->p == 0;
}

int main(void) {
    return check_character() && check_integer() && check_boolean() && check_floating() && check_metadata_portability() ? 0 : 1;
}
