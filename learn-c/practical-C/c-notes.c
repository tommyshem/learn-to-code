#include <stdint.h>
#include <stdio.h>
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
 * direct address example
 */
void direct_address_example() {
  /* example of direct addressing 32 bit chip */
  uint8_t bit_position = 6;
  volatile uint32_t *address = (volatile uint32_t *)678985;
  *address = (1 << bit_position); /* puts bit 1 into the address at bit position
                                     all over bits are unchanged */
}
