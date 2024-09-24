#include <stdio.h>

/*
 *	Exercise 4-3
 *	Ouput result of a rectangle area using width = 3 and 5 for the length
 *	Area = width * length
 *	Output result of a rectangle Perimeter
 *	Perimeter = 2 (width+length)
 *	6.8 x 2.3
 */

/* define values for the rectangles */
#define WIDTH_INT 3
#define LENGTH_INT 5
#define WIDTH_FLOAT 2.3
#define LENGTH_FLOAT 6.8
/* variables */
int result_int;
double result_float;

int main() {
  printf("Area of a rectangle 3 x 5 = %d \n",
         result_int = WIDTH_INT * LENGTH_INT);
  printf("Perimeter of a rectangle 3 x 5 = %d \n",
         result_int = 2 * (WIDTH_INT + LENGTH_INT));
  printf("Area of a rectangle 6.8 x 2.3 = %f\n",
         result_float = WIDTH_FLOAT * LENGTH_FLOAT);
  printf("Perimeter of a rectangle 6.8 x 2.3 = %f \n",
         result_float = 2 * (6.8 + 2.3));
  printf("\n");
}
