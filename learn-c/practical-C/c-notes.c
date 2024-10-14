#include <stdint.h>
#include <stdio.h>

/* static keyword
 * static has 2 meanings
 * static at global scope means variable is local to current file only and are
 * initialized once static at local scope means variables are not temporary and
 * are initialized only once where are temporary variables are initialized every
 * time they go out of scope
 */
/* static example */
void static_example() {
  int counter;
  for (counter = 0; counter < 3; ++counter) {
    int temporary_variable = 1;
    static int permanent_variable = 1;
    printf("Temporary variable = %d permanent variable = %d \n",
           temporary_variable, permanent_variable);
    ++temporary_variable;
    ++permanent_variable;
  }
}

/* structure example */
typedef struct {
  char padding_character;      /* padding to use default is [space] */
  char vert_character;         /* character to use to draw the vert line */
  char horz_character;         /* character to use to draw the horz line */
  int row_height;              /* row height to draw */
  int column_width;            /* column width to draw */
  int columns_horizontal_size; /* size of the checker board width */
  int columns_vertical_size;   /* size of the checker board hight */
} checkerboard;
/* variables */
int result_8_int = 0;
int valid_input_8 = 0;

/*
 * example of the switch statement
 */
void switch_example() {
  int number_to_print = 2;
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

/* strings
 * example of passing in a copy of the string to a function */
int length_of_string(char string[]) {
  int index;
  for (index = 0; string[index] != '\0'; ++index) { /* do nothing*/
  }
  return (index);
}

/*
 * Get valid number from standard input
 */
int _get_value_int_from_input(int *return_result_int_ptr) {
  char string_8[100];
  int result = 0;
  fgets(string_8, sizeof(string_8), stdin);
  result = sscanf(string_8, "%d", return_result_int_ptr);
  return (result);
}

/* example of getting input for numbers */
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
 * Example of direct addressing 32 bit chip 
 */
void direct_address_example() {
 
  uint8_t bit_position = 6;
  volatile uint32_t *address = (volatile uint32_t *)678985;
  *address = (1 << bit_position); /* puts bit 1 into the address at bit position
                                     all over bits are unchanged */
}

/* set a bit using a bit mask */
static inline void set_bit(long *x, int bitNum) {
    *x |= (1L << bitNum);
}

/* clear a bit using a bit mask */
static inline void clear_bit(long *x, int bitNum) {
    *x &=~ (1L << bitNum);
}

/* toggle a bit using a bit mask */
static inline void toggle_bit(long *x, int bitNum) {
    *x ^= (1L << bitNum);
}
