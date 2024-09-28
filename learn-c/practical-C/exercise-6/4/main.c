#include <stdio.h>

/*
 *	Exercise 6-4
 *	calculate money from less than £1 pound into smaller coins 50p 20p 5p 1p
 */

/* Get valid number from standard input */
int _get_value_from_input(char *string_to_use, float *return_result_float_ptr) {
  int result = 0;
  fgets(string_to_use, sizeof(string_to_use), stdin);
  result = sscanf(string_to_use, "%f", return_result_float_ptr);
  return (result);
}

/* get input with min number and max number range*/
float get_input_with_range(int min, int max) {
  /* variables */
  char string[100];
  int valid_input = 0;
  float result_float = 0;
  /* Get a valid percentage 0-100 input from the standard input */
  while ((1)) {
    valid_input = _get_value_from_input(string, &result_float);
    if (!valid_input) {
      printf("Please type in a valid response to the question above !! \n");
    } else { /* check range min to max inputted */
      if (result_float <= max) {
        if (result_float >= min) {
          return (result_float);
        }
        printf("Please type in a valid response to the question above !! \n");
      }
    }
  }
}

int main(void) {
  /* variables */
  float money; /* entered hours to check for overtime */
  int temp;
  /* Input money less than £1 */
  printf("Please enter your money (less than 1): ");
  money = get_input_with_range(0, 1);
  /* check 50p*/
  if ((temp = money / 0.5)) {
    money = money - (temp * 0.5);
    printf("50p x %d\n",temp);
  }
  /* check 20p*/
  if ((temp = money / 0.2)) {
    money = money - (temp * 0.2);
     printf("20p x %d\n",temp);
  }
  /* check 10p*/
  if ((temp = money / 0.1)) {
    money = money - (temp * 0.1);
     printf("10p x %d\n",temp);
  }
  /* check 5p*/
  if ((temp = money / 0.05)) {
    money = money - (temp * 0.05);
     printf("5p x %d\n",temp);
  }
  /* check 1p*/
  if ((temp = money / 0.01)) {
    money = money - (temp * 0.01);
     printf("1p x %d\n",temp);
  }
  printf("Money = %f\n", money);

  /* End program */
  return (0);
}
