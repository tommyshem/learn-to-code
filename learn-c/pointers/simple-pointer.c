#include "stdio.h"

int main (){
int a;
int *p;
p = &a;
a =5;
printf("Value in A = %d\n",a);
printf("Address of A = %p\n",&a);
printf("Address of P %p\n",&p);
printf("P pointers to A address = %p\n",p);
*p = 7;
printf("Value of A = %d\n",a);
}

//         A = 5
//         ^   ^
//    P ----   |
//             |
//   *p -------
