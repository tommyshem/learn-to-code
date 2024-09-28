#include <stdio.h>
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

int main(){
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

	return 0;
}
