#include <stdio.h>

/*
 *	Exercise 8-1
 *	print a checker board 8x8 +-----+-----+
 */
struct checkerboard {
  char padding_character;      /* padding to use default is [space] */
  char vert_character;         /* character to use to draw the vert line */
  char horz_character;         /* character to use to draw the horz line */
  int row_height;              /* row height to draw */
  int column_width;            /* column width to draw */
  int columns_horizontal_size; /* size of the checker board width */
  int columns_vertical_size;   /* size of the checker board hight */
};

/* print padding a number of types */
void _print_padding_character(int width, char pad_character) {
  int i;
  for (i = 0; i < width; ++i) {
    printf("%c", pad_character); /* fill in the row character  */
  }
}
void _print_gap(int columns_horizontal_size, int width) {
  int i;
  for (i = 0; i < columns_horizontal_size; ++i) {
    printf("|");
    _print_padding_character(width, ' ');
  }
  printf("|");
}
void _print_line(int columns_horizontal_size, int width) {
  int i;
  for (i = 0; i < columns_horizontal_size; ++i) {
    printf("+"); /* start of the column on a row */
    _print_padding_character(width, '-');
  }
  printf("+ \n"); /* end of the column on a row*/
}

/* Print the row line */
void print_checker_board(int column_width, int row_height,
                         int columns_vertical_size,
                         int columns_horizontal_size) {
  /* variables */
  int j, t; /* used for looping */
  for (j = 0; j < columns_vertical_size; ++j) {
    _print_line(columns_horizontal_size, column_width);

    /* print the up characters */
    for (t = 0; t < row_height; ++t) {
      _print_gap(columns_horizontal_size, column_width);
      printf("\n");
    }
  }
  _print_line(columns_horizontal_size, column_width);
}

void exercise_8_1(void) {
  /* variables */
  struct checkerboard checkerboard = {.column_width = 10,
                                      .row_height = 5,
                                      .columns_horizontal_size = 3,
                                      .columns_vertical_size = 5,
                                      .padding_character = ' ',
                                      .vert_character = '|',
                                      .horz_character = '-'};
  print_checker_board(10, 5, 2, 3);

}
