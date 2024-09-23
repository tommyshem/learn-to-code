#include <stdio.h>
#include <string.h>
/* 
*	EXAMPLE 5-6
*	Get input with fgets() standard c function
*	note it captures the new line character also in the buffer passed
*/

/* variables */
char first_name[100]; /* Person's first name*/
char last_name[100]; /* Person's last name*/
char full_name[200]; /* Person's first name + last name*/

int main(){
	printf("Enter first name: ");
	fgets(first_name,sizeof(first_name),stdin);  /* Get first name*/
	/* 
	* Trim the last char which is the \n (new line) and replace with \0 
	* which is standard c string termination value (end of string).
	*/
	first_name[strlen(first_name) - 1] = '\0';
	printf("Enter last name: ");
	fgets(last_name,sizeof(last_name),stdin);
	last_name[strlen(last_name) - 1] = '\0';
	/* string copy */
	strcpy(full_name,first_name);
	/* string concatenate */
	strcat(full_name," ");
	strcat(full_name,last_name);
	printf("The name is %s\n",full_name);
	return 0;
}
