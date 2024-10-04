#include <stdio.h>

/*
 *	Exercise 7-4
 *	add 8% tax to price
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
  float price;
  const float tax = 0.08;
  /* Input price */
  printf("Please enter price (1 - 500) : ");
  price = get_input_with_range(1, 500);



  /* End program */
  return (0);
}
