#include <stdio.h>
/* variables */
int t_result_int = 0;
int hours_int = 0;
int minutes_int = 0;
char string[100];
int valid_input = 0;

/* Get valid number from standard input */
int get_value_from_input(void) {
  int result = 0;
  fgets(string, sizeof(string), stdin);
  result = sscanf(string, "%d", &t_result_int);
  return (result);
}
/*
 *	Exercise 4-1 change a bit.
 *   Ouput Name, Fav Hobby, Fav Food on a new line.
 */

void exercise_4_1(void) {
  printf("Name - Tom\n");
  printf("Fave Hobby - Computers\n");
  printf("Fav Food - Cake\n");
}

/*
 *	Exercise 4-2
 *   Ouput block E in * height = 7, width = 5.
 */

void exercise_4_2(void) {
  printf("*****\n");
  printf("*\n");
  printf("*****\n");
  printf("*\n");
  printf("*****\n");
}

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

void exercise_4_3(void) {
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

/*
 *	Exercise 4-4
 *	Ouput HELLO in * with 7 characters height and the width of 5 characters
 */

void exercise_4_4(void) {
  printf("*   * *****  *     *     *****\n");
  printf("*   * *      *     *     *   *\n");
  printf("*   * *      *     *     *   *\n");
  printf("***** ****** *     *     *   *\n");
  printf("*   * *      *     *     *   *\n");
  printf("*   * *      *     *     *   *\n");
  printf("*   * ****** ***** ***** *****\n");
}

/*
 *	Exercise 4-5
 *	Ouput result of wrong types to see the output
 */

/* variables */
int result_int = 5;
double result_float = 2.8;
char result_char = 'Y';

void exercise_4_5(void) {
  printf("Print int type with a float 2.8 passed in = %d \n", result_float);
  printf("Print float type with a int 5 passed in = %f \n", result_int);
  printf("Print int type with a character passed in = %d", result_char);
  printf("\n");
  printf("See the wrong outputs printed with no errors\n");
}

/*
 *	Exercise 5-1
 *	Ouput result of converts centigrade to fahrenheit 
 *     F = 9 C +32
 *         -
 *         5
 * note 9/5 = 1.8 so used this in the formula below.
 */

/* variables */
double results_float = 0;
double centigrade = 25;
double fahrenheit = 0;

void exercise_5_1(void) {
  printf("Convert 25 centigrade to fahrenheit = %f \n", results_float=(1.8*centigrade)+32);
}

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
double result_floats = 0;
double volume = 0;
double sphere_r = 25;
double pie = 3.14159265359;
void exercise_5_2(void) {

  printf("Convert sphere radius 25cm into volume  = %f cm3\n", result_floats=4.18879020477*(25*25*25));
}

/*
 *	Exercise 5-3
 *	Ouput result of perimeter of a rectangle
 *  perimeter = 2 * (width + height)
 *  test width = 5 height = 10 result = 30 
 */

/* variables */
float results_floats = 0.0;
float width_float = 5.0;
float height_float = 10.0;

void exercise_5_3(void) {
  printf("perimeter of a rectangle with a width = 5, height =10 so the perimeter = %f",results_floats = 2*(width_float+height_float));
}

/*
 *	Exercise 5-4
 *	Ouput result by converting kilometers per hour to miles per hour
 *  miles = (kilometers * 0.6213712)
 *  test kilometers = 1500 resut =  
 */

/* variables */
float k_result_float = 0.0;
float kilometer_float = 1500.0;

void exercise_5_4(void) {
  printf("1500 kilometers per hour to miles per hour = %f",k_result_float = (kilometer_float*0.6213712));
}

/*
 *	Exercise 5-5
 *	Ouput result total minutes from input of hours and minutes
 */



void exercise_5_5(void) {

  /* Get the input for hours */
  while ((1)) {
    printf("Input hours: ");
    valid_input = get_value_from_input();
    if (!valid_input) {
      printf("Please type in a valid number !! \n");
    } else {
      break;
    }
  }
  hours_int = result_int;
  /* Get input for minutes*/
  while ((1)) {
    printf("Input minutes (0 - 59): ");
    valid_input = get_value_from_input();
    if (!valid_input) {
      printf("Please type in a valid number !! \n");
    } else {
      if (result_int <= 59) {
        break;
      }
       printf("Please enter valid number 0 - 59\n");
    }
  }
  minutes_int = result_int;
  printf("Total minutes = %d\n", result_int = hours_int * 60 + minutes_int);

}

/*
 *	Exercise 5-6
 *	Ouput the result total hours and minutes from input minutes
 */

/* variables */
int time_result_int = 0;
int time_hours_int = 0;
int time_minutes_int = 0;
char string[100];
int time_valid_input = 0;


void exercise_5_6(void) {

  /* Get the input for minutes */
  while ((1)) {
    printf("Input minutes: ");
    valid_input = get_value_from_input();
    if (!valid_input) {
      printf("Please type in a valid number !! \n");
    } else {
      break;
    }
  }
  /* Calculate the result*/
  hours_int = result_int/60;  /* get hours*/
  minutes_int = result_int-(hours_int*60); /* Get the left over minutes */

  /* Output the result */
  printf("Total hours = %d and minutes %d\n", hours_int,minutes_int);
}

