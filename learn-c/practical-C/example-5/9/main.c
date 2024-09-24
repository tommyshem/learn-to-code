#include <stdio.h>
/*
 *	EXAMPLE 5-9
 *	Example area of a triangle
 */

/* variables */
char input_line[100]; /* input line from the console */
int height;           /* height of the triangle */
int width;            /* width of the triangle */
int area;             /* area of the triangle*/

int main() {
  printf("Enter width of the triangle: ");
  /* Get the input and scan for numbers */
  fgets(input_line, sizeof(input_line), stdin);
  int result = sscanf(input_line, "%d", &width);
  if (!result) {
    printf("You entered an invalid number !\n");
    return (1);
  }
  printf("Enter height of the triangle: ");
  /* Get the input and scan for numbers */
  fgets(input_line, sizeof(input_line), stdin);
  result = sscanf(input_line, "%d", &height);
  if (!result) {
    printf("You entered an invalid number !\n");
    return (2);
  }
  area = (width * height) / 2; /* get the area of the triangle */
  printf("The area is %d\n", area);
  return (0);
}
