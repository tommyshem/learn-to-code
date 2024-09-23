#include <stdio.h>
/* 
*	EXAMPLE 5-8
*	Example how to read in numbers from an input
*	note never use scanf() as it is buggy use fgets() to get the input
*	then scan for the numbers from the input with sscanf()
*/

/* variables */
char input_line[100]; /* input line from the console */
int value; /* a value to double */

int main(){
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
	return 0;
}
