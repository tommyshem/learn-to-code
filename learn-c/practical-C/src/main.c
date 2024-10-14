/* Examples include files */
#include "examples/example13_6.c"
#include "examples/example_13_10.c"
#include "examples/example_13_12.c"
#include "examples/examples_5_to_8.c"

/* Exercises include files */
#include "exercises/exercises_12.c"
#include "exercises/exercises_4_to_5.c"
#include "exercises/exercises_6_to_7.c"
#include "exercises/exercises_8_to_9.c"
/* Standard include files */
#include <stdio.h>

int main(int argc, char *argv[]) {
  /*
   * Examples and Exercises from the Practical C book
   *
   */
  /* Only run the example or exercise you are working on
   */
  // char string[] = {"Hello to you"};
  // exercise_9_1(string);
  example_13_12(argc, argv);
  return (0); /* comment out this return to run all examples and exercises */

  /* all examples are below */
  int exit_code;
  /* Examples 5 from practical C book */
  example_5_6();
  example_5_7();
  example_5_8();
  exit_code = example_5_9();
  printf("Exit code = %d", exit_code);
  /* Examples 6 from practical C book */
  example_6_1();
  /* Examples 8 from practical C book */
  example_8_4();
  /* Examples 8 from practical C book */
  example_13_12(argc, argv);

  /* Exercises 4 from practical C book */
  exercise_4_1();
  exercise_4_2();
  exercise_4_3();
  exercise_4_4();
  exercise_4_5();
  /* Exercises 5 from practical C book */
  exercise_5_1();
  exercise_5_2();
  exercise_5_3();
  exercise_5_4();
  exercise_5_5();
  /* Exercises 6 from practical C book */
  exercise_6_2();
  exercise_6_3();
  exercise_6_4();
  exercise_6_5();
  exercise_6_6();
  /* Exercises 7 from practical C book */
  exercise_7_1();
  exercise_7_4();
  /* Exercises 8 from practical C book */
  exercise_8_1();
  exercise_8_2();
  exercise_8_3();
  exercise_8_4();
  exercise_8_5();
  exercise_8_6();
  exercise_8_7();
  /* Exercise 9 from practical C book */
  exercise_9_1(string);
  /* Exercise 12 from practical C book */
  exercise_12_1();
  exercise_12_2();
  exercise_12_3();
  exercise_12_4();
  /* Exercise 13 from practical C book */

  /* End program */
  return (0);
}
