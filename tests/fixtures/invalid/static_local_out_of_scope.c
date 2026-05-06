int main() {
    {
        static int hidden = 1;
        hidden += 1;
    }
    return hidden;
}
