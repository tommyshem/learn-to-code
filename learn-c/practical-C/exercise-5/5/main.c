#include <stdio.h>

/*
 *	Exercise 5-5
 *	Ouput result total minutes from input of hours and minutes
 */

/* variables */
int result_int = 0;
int hours_int = 0;
int minutes_int = 0;
char string[100];
int valid_input = 0;

int get_value_from_input() {
  fgets(string, sizeof(string), stdin);
  int result = sscanf(string, "%d", &result_int);
  return (result);
}

int main() {
  /* Get the input for hours */
  while ((1)) {
    printf("Input hours: ");
    valid_input = get_value_from_input();
    if (!valid_input) {
      printf("Please type in a valid number !! \n");
    } else {
      break;
    }
  }
  hours_int = result_int;
  /* Get input for minutes*/
  while ((1)) {
    printf("Input minutes (0 - 59): ");
    valid_input = get_value_from_input();
    if (!valid_input) {
      printf("Please type in a valid number !! \n");
    } else {
      if (result_int <= 59) {
        break;
      }
       printf("Please enter valid number 0 - 59\n");
    }
  }
  minutes_int = result_int;
  printf("Total minutes = %d\n", result_int = hours_int * 60 + minutes_int);
  /* End program */
  return (0);
}
