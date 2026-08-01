char *strtok(char *string, const char *delimiters);

int main(void) {
    char text[16] = ";;alpha,beta:";
    char *first = strtok(text, ";,");
    char *second = strtok(0, ",:");
    char *done = strtok(0, ";");

    if (first != text + 2 || first[0] != 'a' || first[4] != 'a' || first[5] != 0) {
        return 1;
    }
    if (second != text + 8 || second[0] != 'b' || second[3] != 'a' || second[4] != 0) {
        return 2;
    }
    if (done != 0 || text[7] != 0 || text[12] != 0) {
        return 3;
    }

    char replacement[8] = "::x:y";
    first = strtok(replacement, ":");
    second = strtok(0, ":");
    done = strtok(0, ":");
    return first == replacement + 2 && first[0] == 'x' && first[1] == 0
            && second == replacement + 4 && second[0] == 'y' && second[1] == 0
            && done == 0
        ? 0
        : 4;
}
