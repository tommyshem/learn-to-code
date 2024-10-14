#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/************************************************************************************
 *
 * my_strchr -- find a character in a string. Duplicated of the standard
 * library function, put here for illustrative purposes
 *@param string_ptr -- string to look through
 *@param find -- character to find in the string
 *@return pointer to 1st occurrence of the character in the string or NULL for
 *error.
 ************************************************************************************/

char *my_strchr(char *string_ptr, char find) {

  while (*string_ptr != find) {
    /* check for end */
    if (*string_ptr == '\0') {
      return (NULL); /* not found */
    }
    ++string_ptr;
  }
  return (string_ptr); /* found character */
}

void example_13_10() {

  char line[80];   /* input buffer */
  char *first_ptr; /* pointer to the first name */
  char *last_ptr;  /* pointer to the second name */
  printf("Input Last name then / first name eg Smith/John : ");
  
  fgets(line, sizeof(line), stdin);
  /* get rid of the trailing newline */
  line[strlen(line) - 1] = '\0';
  last_ptr = line;                  /* last name is at beginning of the line */
  first_ptr = my_strchr(line, '/'); /* find slash in the string */
  /* check for an error */
  if (first_ptr == NULL) {
    fprintf(stderr, "Error: Unable to find slash in %s\n", line);
    exit(8);
  }
  *first_ptr = '\0'; /* zero out the slash to make c style string ending */
  ++first_ptr;       /* move pointer to first character in the split string */
  printf("First name : %s Last name : %s\n", first_ptr, last_ptr);
}
