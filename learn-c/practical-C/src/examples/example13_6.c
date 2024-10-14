#include <stdio.h>
/* pointers and arrays */

/* init_array_1 -- Zeros out an array
 * @param data -- the array to zero out.
 */
const int MAX = 10;

void init_array_1(int data[]) {
  int index;
  for (int index = 0; index < MAX; ++index) {
    data[index] = 0;
  }
  printf("init_array_1 finished zeroing out the passed array");
}

/* pointers and arrays */

/* init_array_2 -- Zeros out an array
 * @param data -- pointer to the array to zero out.
 */
void init_array_2(int *data_ptr) {
  int index;
  for (int index = 0; index < MAX; ++index) {
    *(data_ptr + index) = 0;
  }
  printf("init_array_2 finished zeroing out the passed array");
}

void example_13_6() {
  int array[MAX];
  /* one way of initializing the array */
  init_array_1(array);
  /* another way of initializing the array */
  init_array_1(&array[0]);
  /* works but the compiler generates a warning */
  init_array_1(&array);
  /* similar to the first method but function is different as it uses a pointer
   */
  init_array_2(array);
}

