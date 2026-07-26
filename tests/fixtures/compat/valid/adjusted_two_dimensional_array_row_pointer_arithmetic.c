int mutate_rows(int matrix[][3]) {
  matrix[0][1] += 5;
  matrix[1][2]++;
  return matrix[0][1] + matrix[1][2];
}

int forward_rows(int matrix[][3]) {
  int result = mutate_rows(matrix + 1);
  if ((matrix + 2) - matrix != 2) return 101;
  if (!(matrix + 1 > matrix)) return 102;
  if (matrix + 1 == matrix) return 103;
  return result;
}

int main(void) {
  int values[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};
  if (forward_rows(values) != 20) return 1;
  if (values[1][1] != 10) return 2;
  if (values[2][2] != 10) return 3;
  return 0;
}
