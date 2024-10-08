#include <stdio.h>

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
  printf("total resistance in ohm's = %.2f\n", 1/total_resistance);
}

void exercise_8_3(void) {
  
}