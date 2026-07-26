typedef int Matrix[2][3];
typedef const int ConstMatrix[2][3];

int adjust(Matrix);
int checksum(ConstMatrix);
int chars(char grid[][2]);

int adjust(Matrix matrix) {
  matrix[0][1] += 5;
  matrix[1][2]++;
  return matrix[0][1] + matrix[1][2];
}

int forward(Matrix matrix) {
  return adjust(matrix);
}

int checksum(ConstMatrix matrix) {
  return matrix[0][0] + matrix[1][1];
}

int chars(char grid[][2]) {
  return grid[0][1] + grid[1][0];
}

int main(void) {
  Matrix values = {{1, 2, 3}, {4, 5, 6}};
  ConstMatrix fixed = {{7, 8, 9}, {10, 11, 12}};
  char labels[2][2] = {{'A', 'B'}, {'C', 'D'}};

  if (forward(values) != 14) return 1;
  if (values[0][1] != 7 || values[1][2] != 7) return 2;
  if (checksum(fixed) != 18) return 3;
  if (chars(labels) != 'B' + 'C') return 4;
  return 0;
}
