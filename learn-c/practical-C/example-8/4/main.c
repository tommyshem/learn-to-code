#include <stdio.h>

/*
 *	Example 8-4
 *	count 3 and 7 in an array dataset
 */

int main(void) {
  /* variables */
  char line[100];
  int seven_count = 0;
  int three_count = 0;
  int index;
  int data[5];
  printf("Enter 5 numbers \n");
  /* get input this is the safe way - no buffer overflow */
  fgets(line, sizeof(line), stdin);
  sscanf(line, "%d %d %d %d %d", &data[0], &data[1], &data[2], &data[3],
         &data[4]);

  /* loop over dataset */
  for (index = 0; index < 5; ++index) {
    if (data[index] == 3) {
      ++three_count;
    } else if (data[index] == 7) {
      ++seven_count;
    }
  }
  printf("Threes %d Sevens %d \n", three_count, seven_count);

  /* End program */
  return (0);
}
