#ifdef __STDC__
#include <locale.h>
#endif

int strcmp(const char *left, const char *right);
int strcoll(const char *left, const char *right);
unsigned long int strxfrm(char *restrict destination,
                          const char *restrict source,
                          unsigned long int count);

char *mark_destination(int *calls, char *destination) {
    *calls += 1;
    return destination;
}

const char *mark_source(int *calls, const char *source) {
    *calls += 1;
    return source;
}

unsigned long int mark_count(int *calls, unsigned long int count) {
    *calls += 1;
    return count;
}

int main(void) {
#ifdef __STDC__
    if (setlocale(LC_ALL, "C") == 0) {
        return 6;
    }
#endif
    int destination_calls = 0;
    int source_calls = 0;
    int count_calls = 0;
    int left_calls = 0;
    int right_calls = 0;
    char transformed[8] = {'x', 'x', 'x', 'x', 'x', 'x', 'x', 'x'};
    char compared_left[4] = "cab";
    char compared_right[4] = "cab";

    if ((strcoll(mark_source(&left_calls, compared_left),
                 mark_source(&right_calls, compared_right)) == 0)
            != (strcmp(compared_left, compared_right) == 0)
        || left_calls != 1 || right_calls != 1) {
        return 1;
    }
    if (strcoll("alpha", "alpine") >= 0 || strcoll("zeta", "beta") <= 0) {
        return 2;
    }

    unsigned long int length = strxfrm(
        mark_destination(&destination_calls, transformed),
        mark_source(&source_calls, "cab"), mark_count(&count_calls, 8));
    if (length != 3 || destination_calls != 1 || source_calls != 1
        || count_calls != 1 || transformed[0] != 'c' || transformed[1] != 'a'
        || transformed[2] != 'b' || transformed[3] != 0
        || transformed[4] != 'x') {
        return 3;
    }

    if (strxfrm(0, "measure", 0) != 7) {
        return 4;
    }

    destination_calls = 0;
    source_calls = 0;
    count_calls = 0;
    left_calls = 0;
    right_calls = 0;
    if (sizeof(strcoll(mark_source(&left_calls, compared_left),
                       mark_source(&right_calls, compared_right)))
            != sizeof(int)
        || sizeof(strxfrm(mark_destination(&destination_calls, transformed),
                          mark_source(&source_calls, "cab"),
                          mark_count(&count_calls, 8)))
               != sizeof(unsigned long int)
        || destination_calls != 0 || source_calls != 0 || count_calls != 0
        || left_calls != 0 || right_calls != 0) {
        return 5;
    }

    return 0;
}
