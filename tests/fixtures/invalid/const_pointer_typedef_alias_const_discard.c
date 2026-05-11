typedef const int *ConstIntView;

int main(void) {
    int values[2] = {1, 2};
    ConstIntView view = values;
    int *mutable_view = view;
    return mutable_view[0];
}
