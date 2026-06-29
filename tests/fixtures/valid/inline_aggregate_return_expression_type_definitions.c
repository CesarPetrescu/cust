int struct_from_return(void) {
    return (sizeof(struct ReturnBox { int x; int y; }) == sizeof(struct ReturnBox))
        + ((struct ReturnBox){5, 6}).x
        + ((struct ReturnBox){5, 6}).y;
}

int union_from_return(int flag) {
    return (sizeof(union ReturnChoice { int value; char tag; }) == sizeof(union ReturnChoice))
        + ((union ReturnChoice){flag ? 9 : 3}).value;
}

int main(void) {
    return struct_from_return() + union_from_return(1);
}
