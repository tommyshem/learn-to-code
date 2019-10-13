#include <stdio.h>

int main( int argc, char *argv[] )  {

    printf("Arg 0 = %s\n",argv[0]);  // The path and command name passed everytime
   if( argc == 2 ) {                 // check to see if any arguments passed
      printf("The argument supplied is %s\n", argv[1]);
   }
   else if( argc > 2 ) {
      printf("Too many arguments supplied.\n");
   }
   else {
      printf("One argument expected.\n");
   }
}