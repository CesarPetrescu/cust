int main(void) {
    int assigned = ((int){1} = 5);
    int compounded = ((int){3} += 4);
    int prefixed = ++(int){8};
    int postfixed = (int){9}++;
    char char_assigned = ((char){2} = 6);

    return assigned + compounded + prefixed + postfixed + char_assigned;
}
