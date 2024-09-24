#include <stdio.h>

/*
 *	Exercise 4-5
 *	Ouput result of wrong types to see the output
 */

/* variables */
int result_int = 5;
double result_float = 2.8;
char result_char = 'Y';

int main() {
  printf("Print int type with a float 2.8 passed in = %d \n", result_float);
  printf("Print float type with a int 5 passed in = %f \n", result_int);
  printf("Print int type with a character passed in = %d", result_char);
  printf("\n");
  printf("See the wrong outputs printed with no errors\n");
}
