#include "stdio.h"

int main (){
int a = 5;
int *w; 	//wild pointer if it is not being initialized to anything. These types of pointers are not efficient because they may point to some unknown memory location
void *x = NULL; 	//void pointer no data type can store anything but can crash.
int *p = NULL; 	//null pointer
p = &a;  //store address of variable a in pointer p
printf("Value in A = %d\n",a);
printf("Address of A = %p\n",&a);
// Using pointers
printf("The size of p pointer is: %lu\n",sizeof(p));
printf("Address of P %p\n",&p);
printf("P pointers to A address = %p\n",p);   //accessing the address
printf("Value stored in a variable p is:%d\n",*p);   //accessing the value
*p = 7;  // changing the value stored in A
// Check value changed by using pointers
printf("Value of A = %d\n",a);
}

//         A = 5
//         ^   ^
//    P ----   |
//             |
//   *p -------

// Notes
// * 	Serves 2 purpose
// int *p;                   Declaration of a pointer
// a = *p;                   Returns the value of the referenced variable