int main(void) {
    int *values = (int[]){
        sizeof(struct LitArrayBox { int value; }) == sizeof(struct LitArrayBox),
        [sizeof(struct LitArrayIndex { char tag; }) == sizeof(struct LitArrayIndex)] = sizeof(union LitArrayChoice { int value; char tag; }) == sizeof(union LitArrayChoice),
        ((struct LitArrayPoint { int x; }){5}).x
    };
    struct LitArrayBox box = {7};
    struct LitArrayIndex index = {'A'};
    union LitArrayChoice choice = {11};
    struct LitArrayPoint point = {13};
    return values[0] + values[1] + values[2] + box.value + index.tag + choice.value + point.x;
}
