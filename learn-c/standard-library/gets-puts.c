// clang -std=c17 ./gets-puts.c -o gets-puts
#include <stdio.h>
// Use fgets instead of gets as it is not safe.

int main( ) {

   char str[10];

   printf( "Enter a value :");
   fgets(str, sizeof(str), stdin);  // safe functionuse this one.
   //gets(str);  // unsafe built in function

   printf( "\nYou entered: ");
   puts( str );

   return 0;
}