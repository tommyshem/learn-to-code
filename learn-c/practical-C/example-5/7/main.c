#include <stdio.h>
/*
 *	EXAMPLE 5-7
 *	Example how to use with multidimensional arrays
 */

/* variables */
char number_array[3][2]; /* Array of numbers */

int main() {
  /* Full in number array with data */
  number_array[0][0] = 0 * 10 + 0;
  number_array[0][1] = 0 * 10 + 1;
  number_array[1][0] = 1 * 10 + 0;
  number_array[1][1] = 1 * 10 + 1;
  number_array[2][0] = 2 * 10 + 0;
  number_array[2][1] = 2 * 10 + 1;
  /* Print number array 0*/
  printf("number array [0] [0] = %d \n", number_array[0][0]);
  printf("number array [0] [1] = %d \n", number_array[0][1]);
  /* Print number array 1 */
  printf("number array [1] [0] = %d \n", number_array[1][0]);
  printf("number array [1] [1] = %d \n", number_array[1][1]);
  /* Print number array 2*/
  printf("number array [2] [0] = %d \n", number_array[2][0]);
  printf("number array [2] [1] = %d \n", number_array[2][1]);
  return 0;
}
