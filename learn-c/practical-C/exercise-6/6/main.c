#include <stdio.h>

/*
 *	Exercise 6-6
 *	calculate overtime with hours over 40 at time and half
 */

/* Get valid number from standard input */
int _get_value_from_input(char *string_to_use, int *return_result_int_ptr) {
  int result = 0;
  fgets(string_to_use, sizeof(string_to_use), stdin);
  result = sscanf(string_to_use, "%d", return_result_int_ptr);
  return (result);
}

/* get input with min number and max number range*/
int get_input_with_range(int min, int max) {
  /* variables */
  char string[100];
  int valid_input = 0;
  int result_int = 0;
  /* Get a valid percentage 0-100 input from the standard input */
  while ((1)) {
    valid_input = _get_value_from_input(string, &result_int);
    if (!valid_input) {
      printf("Please type in a valid response to the question above !! \n");
    } else { /* check range min to max inputted */
      if (result_int <= max) {
        if (result_int >= min) {
          return (result_int);
        }
        printf("Please type in a valid response to the question above !! \n");
      }
    }
  }
}

int main(void) {
  /* variables */
  int hours_worked; /* entered hours to check for overtime */
  float overtime_hours_worked;
  const float overtime_rate = 1.5;
  /* INput hours worked this week*/
  printf("Please enter your hours worked this week: ");
  hours_worked = get_input_with_range(0, 100);
  /* check if more than 40 hours have been worked */
  if (hours_worked > 40) {
    hours_worked = hours_worked - 40;
    overtime_hours_worked = hours_worked * overtime_rate;
    printf("total Overtime hours worked x 1.5 to normal hours %f\n",overtime_hours_worked);
  } else
    printf("No overtime worked this week \n");

  /* End program */
  return (0);
}
