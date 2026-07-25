int global_matrix[2][3] = {{1, 2, 3}, {4, 5}};
char global_letters[2][2] = {{'A'}, {'B', 'C'}};
static int file_static_matrix[1][2] = {{6, 7}};

int main(void) {
  static int local_static_matrix[2][1] = {{8}, {9}};
  int local_matrix[2][3] = {{10, 11}, {12}};
  int before;
  int after;

  global_matrix[1][2] = 13;
  global_matrix[1][2] += 2;
  local_matrix[1][1] = 4;
  before = local_matrix[1][1]++;
  after = ++local_matrix[1][1];
  global_letters[1][1] = 'Z';

  return global_matrix[0][2] + global_matrix[1][2]
      + local_matrix[0][0] + local_matrix[1][2]
      + before + after + global_letters[0][0] + global_letters[1][1]
      + file_static_matrix[0][1] + local_static_matrix[0][0]
      + (sizeof(global_matrix) == 6 * sizeof(int))
      + (sizeof(global_letters) == 4 * sizeof(char));
}
