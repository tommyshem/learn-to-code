#include <stdio.h>

/* print numbers to words helper function*/
void print_number_to_word(int number_to_print) {
  switch (number_to_print) {
  case 1: {
    printf("one");
    break;
  }
  case 2: {
    printf("two");
    break;
  }
  case 3: {
    printf("three");
    break;
  }
  case 4: {
    printf("four");
    break;
  }
  case 5: {
    printf("five");
    break;
  }
  case 6: {
    printf("six");
    break;
  }
  case 7: {
    printf("seven");
    break;
  }
  case 8: {
    printf("eight");
    break;
  }
  case 9: {
    printf("nine");
    break;
  }
  case 0: {
    printf("zero");
    break;
  }
  }
}
/* variables */
int result_8_int = 0;

int valid_input_8 = 0;
/* Get valid number from standard input */
int _get_value_int_from_input(int *return_result_int_ptr) {
  char string_8[100];
  int result = 0;
  fgets(string_8, sizeof(string_8), stdin);
  result = sscanf(string_8, "%d", return_result_int_ptr);
  return (result);
}
/* get input with min number and max number range*/
int get_input_with_int_range(int min, int max) {
  /* variables */
  int valid_input = 0;
  int result_int = 0;
  /* Get a valid percentage 0-100 input from the standard input */
  while ((1)) {
    valid_input = _get_value_int_from_input(&result_int);
    if (!valid_input) {
      printf("Please type in a valid response to the question above !! \n");
    } else { /* check range min to max inputted */
      if (result_int <= max) {
        if (result_int >= min) {
          return (result_int);
        }
        printf("Please type in a valid response to the question above !! \n");
      }
    }
  }
}

/*
 *	Exercise 8-1
 *	print a checker board 8x8 +-----+-----+
 */
typedef struct {
  char padding_character;      /* padding to use default is [space] */
  char vert_character;         /* character to use to draw the vert line */
  char horz_character;         /* character to use to draw the horz line */
  int row_height;              /* row height to draw */
  int column_width;            /* column width to draw */
  int columns_horizontal_size; /* size of the checker board width */
  int columns_vertical_size;   /* size of the checker board hight */
} checkerboard;

/* print padding a number of types */
void _print_padding_character(checkerboard *checkerboard, int mode) {
  int i;
  for (i = 0; i < checkerboard->column_width; ++i) {
    if (mode == 0) {
      printf("%c",
             checkerboard->padding_character); /* fill in the row character  */
    } else
      printf("%c",
             checkerboard->horz_character); /* fill in the row character  */
  }
}
void _print_gap(checkerboard *checkerboard) {
  int i;
  for (i = 0; i < checkerboard->columns_horizontal_size; ++i) {
    printf("|");
    _print_padding_character(checkerboard, 0);
  }
  printf("|");
}
void _print_line(checkerboard *checkerboard) {
  int i;
  for (i = 0; i < checkerboard->columns_horizontal_size; ++i) {
    printf("+"); /* start of the column on a row */
    _print_padding_character(checkerboard, 1);
  }
  printf("+ \n"); /* end of the column on a row*/
}

/* Print the row line */
void print_checker_board(checkerboard *checkerboard) {
  /* variables */
  int j, t; /* used for looping */
  for (j = 0; j < checkerboard->columns_vertical_size; ++j) {
    _print_line(checkerboard);

    /* print the up characters */
    for (t = 0; t < checkerboard->row_height; ++t) {
      _print_gap(checkerboard);
      printf("\n");
    }
  }
  _print_line(checkerboard);
}

void exercise_8_1(void) {
  /* variables */
  checkerboard checkerboard = {.column_width = 10,
                               .row_height = 2,
                               .columns_horizontal_size = 5,
                               .columns_vertical_size = 5,
                               .padding_character = ' ',
                               .vert_character = '|',
                               .horz_character = '-'};
  print_checker_board(&checkerboard);
}

/* Exercise 8-2
 *   Calculate total resistance of n resisters in parallel
 *   1   1   1   1
 *   - = - + - + -   .......
 *   R   R1  R2  R3
 */

void exercise_8_2(void) {
  /* variables */
  int number_parallel_resisters;
  float total_resistance = 0;
  float resistance = 0;
  float resister[10] = {0};
  int i;
  /* input the number of parallel resisters to calculate  */
  printf("Input the number of parallel resisters to calculate: (1 - 10) ");
  number_parallel_resisters = get_input_with_int_range(1, 10);
  /* get input for each resister in ohm's */
  for (i = 0; i < number_parallel_resisters; ++i) {
    printf("Enter resister %d value in ohm's (1 - 100,000 ohm) :", i + 1);
    resister[i] = get_input_with_int_range(1, 100000);
  }
  /* Calculate */
  for (i = 0; i < number_parallel_resisters; ++i) {
    resistance = 1.0 / resister[i];
    total_resistance = total_resistance + resistance;
  }
  printf("total resistance in ohm's = %.2f\n", 1 / total_resistance);
}
/* Exercise 8-3 average numbers n numbers*/
void exercise_8_3(void) {
  int number = 0;
  int numbers[100] = {0};
  int i = 0;
  int total = 0;
  /* Get input for total numbers to average from the std input */
  printf("Input number of numbers to average (1-100) : ");
  number = get_input_with_int_range(1, 100);
  /* get the actual numbers to average from the std input*/
  for (i = 0; i < number; ++i) {
    printf("Input number 1 to 10000 : ");
    numbers[i] = get_input_with_int_range(1, 10000);
  }
  /* Calculate the average of the numbers entered */
  for (i = 0; i < number; ++i) {
    total = total + numbers[i];
  }
  /* Display the average */
  printf("Average for the numbers entered = %d\n", total / number);
}

/* Exercise 8-4 print out the multiplication table */
void exercise_8_4(void) {
  int i = 0;
  int j = 0;
  for (j = 1; j < 13; j = j + 4) {
    for (i = 1; i < 13; ++i) {
      printf("     %3d x %3d = %3d", i, j, i * j);
      printf("     %3d x %3d = %3d", i, j + 1, i * (j + 1));
      printf("     %3d x %3d = %3d", i, j + 2, i * (j + 2));
      printf("     %3d x %3d = %3d\n", i, j + 3, i * (j + 3));
    }
    printf("\n");
  }
}

/* Exercise 8-5 read a character and print out if its a vowel or a consonant
 * vowels are a,e,i,o,u
 */
void exercise_8_5(void) {
  char character;
  character = getchar();
  switch (character) {
  case 'a':
  case 'e':
  case 'i':
  case 'o':
  case 'u': {
    printf("Vowel\n");
    break;
  }
  default:
    printf("consonant\n");
  }
}
/* Exercise 8-6 convert numbers to words */
void exercise_8_6(void) {
  int number;
  int result;
  int zero;
  /* get number from std input to convert to words */
  printf("Enter a number 1 to 9000 : ");
  number = get_input_with_int_range(1, 9000);
  /* convert number */
  if (number >= 1000) {
    result = number / 1000;
    print_number_to_word(result);
    printf("-");
    number = number - (result * 1000);
  }
  if (number >= 100) {
    result = number / 100;
    print_number_to_word(result);
    printf("-");
    number = number - (result * 100);
  }
  if (number >= 10) {
    result = number / 10;
    print_number_to_word(result);
    printf("-");
    number = number - (result * 10);
  }
  print_number_to_word(number);
  printf("\n");
}

/* Exercise 8-7 modify 8-6 so it ouput's eg 85 = eighty-five instead of
 * eight-five numbers 0-100 */
void exercise_8_7(void) {
  int number;
  int result;
  int zero;
  /* get number from std input to convert to words */
  printf("Enter a number 1 to 9000 : ");
  number = get_input_with_int_range(1, 9000);

  /* convert thousands to words */
  if (number >= 1000) {
    result = number / 1000;
    print_number_to_word(result);
    printf(" thousand ");
    number = number - (result * 1000);
  }

  /* convert hundreds to words */
  if (number >= 100) {
    result = number / 100;
    print_number_to_word(result);
    printf(" hundred ");
    number = number - (result * 100);
  }

  /* convert tens to words */
  if (number >= 20) {
    result = number / 10;
    /* tens to words */
    switch (result) {
    case 2: {
      printf("twenty");
      break;
    }
    case 3: {
      printf("thirty");
      break;
    }
    case 4: {
      printf("forty");
      break;
    }
    case 5: {
      printf("fifty");
      break;
    }
    case 6: {
      printf("sixty");
      break;
    }
    case 7: {
      printf("seventy");
      break;
    }
    case 8: {
      printf("eighty");
      break;
    }
    case 9: {
      printf("ninety");
      break;
    }

    } /* end switch statement */
    number = number - (result * 10);
  } /* end if statement */

  /* numbers 19 downwards to numbers 10 to words */
  printf(" "); /* format with a space */
  if (number >= 10) {
    switch (number) {
    case 19: {
      printf("nineteen");
      break;
    }
    case 18: {
      printf("eighteen");
      break;
    }
    case 17: {
      printf("seventeen");
      break;
    }
    case 16: {
      printf("sixteen");
      break;
    }
    case 15: {
      printf("fifteen");
      break;
    }
    case 14: {
      printf("fourteen");
      break;
    }
    case 13: {
      printf("thirteen");
      break;
    }
    case 12: {
      printf("twelve");
      break;
    }
    case 11: {
      printf("eleven");
      break;
    }
    case 10: {
      printf("ten");
      break;
    }
    }
    number = 0;
  }
  /* less than number 9 to words */
  if (number > 0) {
    print_number_to_word(number);
  }
  printf("\n"); /* format end of line */
}

/* Exercise 9-1 
* counts the number of words from the string passed in the parameter
* words are counted with a space and letters on each side.
* @param char string[] -- the string from which to count the words from 
* @return int -- number of words from the string passed in
*/
int exercise_9_1(char string[]){
int word_count=0;
int index=0;

return (word_count);
}

/* Exercise 9-2 
* Write a function begins (string1,string2) returns true if string1 begins in string2 
* compare -- compare first string with 2nd string to see it it begins with the same characters 
* @param string -- first string 
* @param string -- second string to compare with first string
*/

/* Exercise 9-3 
* Write a function count (number,array,length) that counts the number of times
* number appears in the array. This function should be recursive. 
*/

/* Exercise 9-4 
* Write a function that takes a character array and returns a primitive hash code
* by adding up the value of each character in the array 
*/

/* Exercise 9-5 
* Write a function that returns the maximum value of an array of numbers 
*
* @param int [] -- int array to find the maximum value from
* @return int -- maximum number in the array
*/

/* Exercise 9-6 write a function that scans a character array for the character and replace it with _ 
*/

