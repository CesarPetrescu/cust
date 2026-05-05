int classify(int x) {
    int score = 0;
    switch (x) {
    case 0:
        score += 2;
        break;
    case 1:
        score += 3;
        // fall through
    case 2:
        score += 5;
        break;
    default:
        score += 7;
    }
    return score;
}

int main() {
    int total = classify(0) + classify(1) + classify(2) + classify(9);
    int i = 0;
    for (i = 0; i < 5; i++) {
        switch (i & 3) {
        case 0:
            total += 10;
            break;
        case 1:
            i++;
            continue;
        case 3:
            break;
        default:
            total += 1;
        }
        total += 2;
    }
    return total;
}
