#include <stdio.h>

/*
 *	Exercise 5-6
 *	Ouput the result total hours and minutes from input minutes
 */

/* variables */
int result_int = 0;
int hours_int = 0;
int minutes_int = 0;
char string[100];
int valid_input = 0;

/* Get valid number from the input */
int get_value_from_input() {
  fgets(string, sizeof(string), stdin);
  int result = sscanf(string, "%d", &result_int);
  return (result);
}

int main() {
  /* Get the input for minutes */
  while ((1)) {
    printf("Input minutes: ");
    valid_input = get_value_from_input();
    if (!valid_input) {
      printf("Please type in a valid number !! \n");
    } else {
      break;
    }
  }
  /* Calculate the result*/
  hours_int = result_int/60;  /* get hours*/
  minutes_int = result_int-(hours_int*60); /* Get the left over minutes */

  /* Output the result */
  printf("Total hours = %d and minutes %d\n", hours_int,minutes_int);
  /* End program */
  return (0);
}
