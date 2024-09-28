#include <stdio.h>

/*
 *	Exercise 5-2
 *	Ouput result of volume of a sphere 
 *     4 pie r 3
 *     -
 *     3
 * note 4/3 * pie = 1.3333333333333 * 3.14159265359 = 4.18879020477 so used this in the formula below.
 * testing input 25 output 65449.8
 */

/* variables */
double result_float = 0;
double volume = 0;
double sphere_r = 25;
double pie = 3.14159265359;
int main() {

  printf("Convert sphere radius 25cm into volume  = %f cm3\n", result_float=4.18879020477*(25*25*25));
  return(0); /* End program with no errors */
}
