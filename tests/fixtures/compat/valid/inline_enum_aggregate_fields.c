struct Flags {
    enum State { STATE_READY = 3, STATE_DONE = 7 } state;
    enum { MODE_FAST = 11 } mode;
};

struct InlineHolder {
    enum Local { LOCAL_BASE = MODE_FAST + 2 } local;
};

struct InlineHolder global_holder = {LOCAL_BASE};

typedef struct {
    enum { TYPE_VALUE = LOCAL_BASE + 4 } code;
} TypeHolder;

TypeHolder typed = {TYPE_VALUE};

int main(void) {
    struct Flags flags = {STATE_DONE, MODE_FAST};
    flags.state = STATE_READY;
    return flags.state + flags.mode + global_holder.local + typed.code;
}
