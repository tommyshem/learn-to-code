#include <stdio.h>

/*
 *	Exercise 6-2
 *	Input numbering grade and output the correct grade
 *  0-60   = F
 *  61-70  = D
 *  71-80  = C
 *  81-90  = B
 *  91-100 = A
 */

/* variables */
int result_int = 0;
char string[100];
int valid_input = 0;

/* Get valid number from standard input */
int get_value_from_input(void) {
  int result = 0;
  fgets(string, sizeof(string), stdin);
  result = sscanf(string, "%d", &result_int);
  return (result);
}

int main(void) {
  /* Get a valid percentage 0-100 input from the standard input */
  while ((1)) {
    printf("Input point a: ");
    valid_input = get_value_from_input();
    if (!valid_input) {
      printf("Please type in a valid number !! \n");
    } else { /* check range 0 to 100 grade inputted */
      if (valid_input <= 100) {
        if (valid_input >= 0) {
          break;
        }
      }
    }
  }

  /* Check percentage and output the correct grade to standard output */
  if (result_int <= 60) {
    printf("Your grade is F\n");
  } else if (result_int <= 70) {
    printf("Your grade is D\n");
  } else if (result_int <= 80) {
    printf("Your grade is C\n");
  } else if (result_int <= 90) {
    printf("Your grade is B\n");
  } else if (result_int <= 100) {
    printf("Your grade is A\n");
  }

  /* End program */
  return (0);
}
