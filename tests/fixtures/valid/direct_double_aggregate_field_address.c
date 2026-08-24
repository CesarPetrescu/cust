struct Sample {
    double reading;
};

int main(void) {
    struct Sample sample = {1.25};
    return &sample.reading != 0;
}
