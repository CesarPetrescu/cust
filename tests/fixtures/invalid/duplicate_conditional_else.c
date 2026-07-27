#define ENABLED
#ifdef ENABLED
int main(void) { return 0; }
#else
int main(void) { return 1; }
#else
int main(void) { return 2; }
#endif
