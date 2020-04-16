// Example:  static local value
// compile with 
// gcc static.c -o static
#include <stdio.h>

void addChicken(){
	static int chickens = 0; // static local value will start with 0 but will hold the changes when called again.
	int eggs = 0; // local value
	chickens += 1;  // add one to the static value.
	eggs +=1;  // add one to the local value
	printf("chickens = %d -> eggs = %d\n",chickens,eggs);
}

int main(){
	addChicken();
	addChicken();
	addChicken();
	return 0;
}

/*
Output should be

chickens = 1 -> eggs = 1
chickens = 2 -> eggs = 1
chickens = 3 -> eggs = 1

Note chickens keeps adding to the static local value but the other local value
keeps reseting the value to 0 everytime the function is called
*/
