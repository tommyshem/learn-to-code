#include <stdio.h>

/*
 *	Exercise 5-1
 *	Ouput result of converts centigrade to fahrenheit 
 *     F = 9 C +32
 *         -
 *         5
 * note 9/5 = 1.8 so used this in the formula below.
 */

/* variables */
double result_float = 0;
double centigrade = 25;
double fahrenheit = 0;

int main() {
  printf("Convert 25 centigrade to fahrenheit = %f \n", result_float=(1.8*centigrade)+32);
  return(0); /* End program with no errors */
}
