#include <stdio.h>
#include <string.h>
/*
 *	EXAMPLE 5-6
 *	Get input with fgets() standard c function
 *	note it captures the new line character also in the buffer passed
 */

/* variables */
char first_name[100]; /* Person's first name*/
char last_name[100];  /* Person's last name*/
char full_name[200];  /* Person's first name + last name*/

void example_5_6(void) {
  printf("Enter first name: ");
  fgets(first_name, sizeof(first_name), stdin); /* Get first name*/
  /*
   * Trim the last char which is the \n (new line) and replace with \0
   * which is standard c string termination value (end of string).
   */
  first_name[strlen(first_name) - 1] = '\0';
  printf("Enter last name: ");
  fgets(last_name, sizeof(last_name), stdin);
  last_name[strlen(last_name) - 1] = '\0';
  /* string copy */
  strcpy(full_name, first_name);
  /* string concatenate */
  strcat(full_name, " ");
  strcat(full_name, last_name);
  printf("The name is %s\n", full_name);
}

/*
 *	EXAMPLE 5-7
 *	Example how to use with multidimensional arrays
 */

/* variables */
char number_array[3][2]; /* Array of numbers */

void example_5_7(void) {
  /* Full in number array with data */
  number_array[0][0] = 0 * 10 + 0;
  number_array[0][1] = 0 * 10 + 1;
  number_array[1][0] = 1 * 10 + 0;
  number_array[1][1] = 1 * 10 + 1;
  number_array[2][0] = 2 * 10 + 0;
  number_array[2][1] = 2 * 10 + 1;
  /* Print number array 0*/
  printf("number array [0] [0] = %d \n", number_array[0][0]);
  printf("number array [0] [1] = %d \n", number_array[0][1]);
  /* Print number array 1 */
  printf("number array [1] [0] = %d \n", number_array[1][0]);
  printf("number array [1] [1] = %d \n", number_array[1][1]);
  /* Print number array 2*/
  printf("number array [2] [0] = %d \n", number_array[2][0]);
  printf("number array [2] [1] = %d \n", number_array[2][1]);
}

/* 
*	EXAMPLE 5-8
*	Example how to read in numbers from an input
*	note never use scanf() as it is buggy use fgets() to get the input
*	then scan for the numbers from the input with sscanf()
*/

/* variables */
char input_line[100]; /* input line from the console */
int value; /* a value to double */

void example_5_8(void){
	printf("Enter a value: ");
	/* Get the input and scan for numbers */
	fgets(input_line,sizeof(input_line),stdin);
	int result = sscanf(input_line,"%d",&value);
	if (result){
	printf("Double %d is %d\n",value,value*2); /* double the value */
	}
	else
	{
		printf("You entered an invalid number !\n");
	}
}

/*
 *	EXAMPLE 5-9
 *	Example area of a triangle
 */

/* variables */
char input_line[100]; /* input line from the console */
int height;           /* height of the triangle */
int width;            /* width of the triangle */
int area;             /* area of the triangle*/

int example_5_9(void) {
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

/* 
*	EXAMPLE 6-1
*	Example how to do fibonacci numbers
*	fn=fn-1+fn-2
*	next_number = current_number + old_number
*/

/* variables */
int old_number; /* previous fibonacci number */ 
int current_number; /* current fibonacci number */
int next_number; /* next number in the fibonacci series */

void example_6_1(void){
	/* Start from 1 */
	old_number = 1;
	current_number =1;

	printf("1\n"); /* Print first number */

	while (current_number <100){
		printf("%d\n",current_number);
	next_number=current_number+old_number;
	old_number=current_number;
	current_number=next_number;

	}
}

/*
 *	Example 8-4
 *	count 3 and 7 in an array dataset
 */
void example_8_4(void) {
  /* variables */
  char line[100];
  int seven_count = 0;
  int three_count = 0;
  int index;
  int data[5];
  printf("Enter 5 numbers \n");
  /* get input this is the safe way - no buffer overflow */
  fgets(line, sizeof(line), stdin);
  sscanf(line, "%d %d %d %d %d", &data[0], &data[1], &data[2], &data[3],
         &data[4]);

  /* loop over dataset */
  for (index = 0; index < 5; ++index) {
    if (data[index] == 3) {
      ++three_count;
    } else if (data[index] == 7) {
      ++seven_count;
    }
  }
  printf("Threes %d Sevens %d \n", three_count, seven_count);
}
