#include <stdio.h>

/*
 *	Exercise 6-2
 *	Input numbering grade and output the correct grade
 *  0-60   = F
 *  61-70  = D
 *  71-80  = C
 *  81-90  = B
 *  91-100 = A
 */

/* variables */
int result_6_int = 0;
char string_6[100];
int valid_input_6 = 0;
/* Get valid number from standard input */
int _get_value_from_input(char *string_to_use, float *return_result_float_ptr) {
  int result = 0;
  fgets(string_to_use, sizeof(string_to_use), stdin);
  result = sscanf(string_to_use, "%f", return_result_float_ptr);
  return (result);
}

/* get input with min number and max number range*/
float get_input_with_range(int min, int max) {
  /* variables */
  char string[100];
  int valid_input = 0;
  float result_float = 0;
  /* Get a valid percentage 0-100 input from the standard input */
  while ((1)) {
    valid_input = _get_value_from_input(string, &result_float);
    if (!valid_input) {
      printf("Please type in a valid response to the question above !! \n");
    } else { /* check range min to max inputted */
      if (result_float <= max) {
        if (result_float >= min) {
          return (result_float);
        }
        printf("Please type in a valid response to the question above !! \n");
      }
    }
  }
}

/* Get valid number from standard input */
int get_value_from_input_6(void) {
  int result_6 = 0;
  fgets(string_6, sizeof(string_6), stdin);
  result_6 = sscanf(string_6, "%d", &result_6_int);
  return (result_6);
}

void exercise_6_2(void) {
  /* Get a valid percentage 0-100 input from the standard input */
  while ((1)) {
    printf("Input point a: ");
    valid_input_6 = get_value_from_input_6();
    if (!valid_input_6) {
      printf("Please type in a valid number !! \n");
    } else { /* check range 0 to 100 grade inputted */
      if (valid_input_6 <= 100) {
        if (valid_input_6 >= 0) {
          break;
        }
      }
    }
  }

  /* Check percentage and output the correct grade to standard output */
  if (result_6_int <= 60) {
    printf("Your grade is F\n");
  } else if (result_6_int <= 70) {
    printf("Your grade is D\n");
  } else if (result_6_int <= 80) {
    printf("Your grade is C\n");
  } else if (result_6_int <= 90) {
    printf("Your grade is B\n");
  } else if (result_6_int <= 100) {
    printf("Your grade is A\n");
  }
}

/*
 *	Exercise 6-3
 *	Input numbering grade and output the correct grade
 *  0-60   = F
 *  61-70  = D
 *  71-80  = C
 *  81-90  = B
 *  91-100 = A
 * credit symbol
 *  1-3 print -
 *  4-7 print nothing
 *  8-0 print +
 * eg 68 = D+
 */

void print_credit_symbol(int grade) {
  grade = grade % 10;
  if (grade <= 3) {
    printf("-\n");
  } else if (grade >= 7) {
    printf("+\n");
  } else {
    printf("\n");
  }
}

void exercise_6_3(void) {
  /* Get a valid percentage 0-100 input from the standard input */
  while ((1)) {
    printf("Input point a: ");
    valid_input_6 = get_value_from_input_6();
    if (!valid_input_6) {
      printf("Please type in a valid number !! \n");
    } else { /* check range 0 to 100 grade inputted */
      if (valid_input_6 <= 100) {
        if (valid_input_6 >= 0) {
          break;
        }
      }
    }
  }

  /* Check percentage and output the correct grade to standard output */
  if (result_6_int <= 60) {
    printf("Your grade is F");
    print_credit_symbol(result_6_int);
  } else if (result_6_int <= 70) {
    printf("Your grade is D");
    print_credit_symbol(result_6_int);
  } else if (result_6_int <= 80) {
    printf("Your grade is C");
    print_credit_symbol(result_6_int);
  } else if (result_6_int <= 90) {
    printf("Your grade is B");
    print_credit_symbol(result_6_int);
  } else if (result_6_int <= 100) {
    printf("Your grade is A");
    print_credit_symbol(result_6_int);
  }
}

/*
 *	Exercise 6-4
 *	calculate money from less than £1 pound into smaller coins 50p 20p 5p 1p
 */

void exercise_6_4(void) {
  /* variables */
  float money; /* entered hours to check for overtime */
  int temp;
  /* Input money less than £1 */
  printf("Please enter your money (less than 1): ");
  money = get_input_with_range(0, 1);
  /* check 50p*/
  if ((temp = money / 0.5)) {
    money = money - (temp * 0.5);
    printf("50p x %d\n", temp);
  }
  /* check 20p*/
  if ((temp = money / 0.2)) {
    money = money - (temp * 0.2);
    printf("20p x %d\n", temp);
  }
  /* check 10p*/
  if ((temp = money / 0.1)) {
    money = money - (temp * 0.1);
    printf("10p x %d\n", temp);
  }
  /* check 5p*/
  if ((temp = money / 0.05)) {
    money = money - (temp * 0.05);
    printf("5p x %d\n", temp);
  }
  /* check 1p*/
  if ((temp = money / 0.01)) {
    money = money - (temp * 0.01);
    printf("1p x %d\n", temp);
  }
  printf("Money = %f\n", money);
}

/*
 *	Exercise 6-5
 *	leap year is devisable by 4 unless the year is devisable by 100 but not
 *  400 calculate if a leap year.
 */

void exercise_6_5(void) {
  int year; /* year to test for a leap year*/
  printf("Please enter a year to check for a leap year: ");
  year = get_input_with_range(1000, 6000);
  /* leap year is devisable by 4 */
  if (year % 4 != 0) {
    printf("Not a Leap Year\n");
    /*unless the year is devisable by 100 but not 400 */
  } else if (year % 400 == 0) {
    printf("Leap Year\n");
  } else if (year % 100 == 0) {
    printf("Not a leap year\n");
  } else {
    printf("Leap year\n");
  }
}

/*
 *	Exercise 6-6
 *	calculate overtime with hours over 40 at time and half
 */

void exercise_6_6(void) {
  /* variables */
  int hours_worked; /* entered hours to check for overtime */
  float overtime_hours_worked;
  const float overtime_rate = 1.5;
  /* INput hours worked this week*/
  printf("Please enter your hours worked this week: ");
  hours_worked = get_input_with_range(0, 100);
  /* check if more than 40 hours have been worked */
  if (hours_worked > 40) {
    hours_worked = hours_worked - 40;
    overtime_hours_worked = hours_worked * overtime_rate;
    printf("total Overtime hours worked x 1.5 to normal hours %f\n",
           overtime_hours_worked);
  } else
    printf("No overtime worked this week \n");
}


/*
 *	Exercise 7-1
 *	convert english units to metric
 *  miles to km
 *  gallons to litres
 */

void exercise_7_1(void) {
  /* variables */
  int hours_worked; /* entered hours to check for overtime */
  float overtime_hours_worked;
  const float overtime_rate = 1.5;
  /* INput hours worked this week*/
  printf("Please enter your hours worked this week: ");
  hours_worked = get_input_with_range(0, 100);
  /* check if more than 40 hours have been worked */
  if (hours_worked > 40) {
    hours_worked = hours_worked - 40;
    overtime_hours_worked = hours_worked * overtime_rate;
    printf("total Overtime hours worked x 1.5 to normal hours %f\n",overtime_hours_worked);
  } else
    printf("No overtime worked this week \n");

}



/*
 *	Exercise 7-4
 *	add 8% tax to price
 */

void exercise_7_4(void) {
  /* variables */
  float price;
  const float tax = 0.08;
  /* Input price */
  printf("Please enter price (1 - 500) : ");
  price = get_input_with_range(1, 500);

/* TODO */
}
