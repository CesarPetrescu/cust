int main(void) {
return _Generic((const int *)0, const int *: 1, int const *: 2);
}
