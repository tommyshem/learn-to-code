#include <stdio.h>

/*
 *	Exercise 6-5
 *	leap year is devisable by 4 unless the year is devisable by 100 but not
 *  400 calculate if a leap year.
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
  int year; /* year to test for a leap year*/
  printf("Please enter a year to check for a leap year: ");
  year = get_input_with_range(1000, 6000);
  /* leap year is devisable by 4 */
  if (year % 4 != 0) {
    printf("Not a Leap Year\n");
    /*unless the year is devisable by 100 but not 400 */
  } else if (year % 400 == 0) {
    printf("Leap Year\n");
  } else if (year % 100 == 0) {
    printf("Not a leap year\n");
  } else {
    printf("Leap year\n");
  }
  /* End program */
  return (0);
}
