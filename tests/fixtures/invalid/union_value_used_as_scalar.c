union Number { int value; char tag; };

int main(void) {
    return (union Number){4};
}
