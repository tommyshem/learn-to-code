#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <time.h>
#include <unistd.h>

const int O_BINARY = 0;
struct fileinfo {
  char filename[30];      /* filename of the file */
  char buffer[16 * 1024]; /* 16k buffer size */
  FILE *in_file;          /* input file descriptor */
  FILE *out_file;         /* output file descriptor*/
  int read_size;          /* number of bytes on the last read*/
};

/*
 * helper file functions
 */
int open_file_unbuffered(struct fileinfo *file_info) {
  int return_status = 1;
  char file_name[] = "test.data";
  file_info->in_file = fopen(file_name, "r");
  if (file_info->in_file == NULL) {
    return_status = 0;
  }
  return (return_status);
}

void close_file_unbuffered(struct fileinfo *file_info) {
  fclose(file_info->in_file);
}
/* Exercise 14_1 write a function that reads in a file and then counts number of
 * lines in it. */

void exercise_14_1() {
  struct fileinfo file_info;
  int file_opened = 0;
  // file_info.filename = "test_file.data";
  file_opened = open_file_unbuffered(&file_info);
  if (file_opened == 0) {
    printf("Error: File %s could not be opened.", file_info.filename);
    exit(8);
  }

  char string[255];
  char *string_ptr;
  int lines = 0;

  while (1) {
    string_ptr = fgets(string, sizeof(string), file_info.in_file);
    if (string_ptr == NULL) {
      break;
    }
    ++lines;
  }
  printf("Lines %d", lines);
}
/* another way */
int exercise_14_1_b() {

  FILE *file;
  char c;
  int lineCount = 0;

  file = fopen("test.data", "r"); // Open file for reading
  if (file == NULL) {
    printf("Error opening file.\n");
    return 1;
  }

  // Read and display contents while counting lines
  while ((c = fgetc(file)) != EOF) {
    if (c == '\n') {
      lineCount++;
    }
    printf("%c", c);
  }

  fclose(file); // Close the file

  printf("\nNumber of lines read: %d\n", lineCount);

  return 0;
}

/* Exercise 14_2 write a function that to copy a file, expanding the tabs to
 * spaces. */
int exercise_14_2() {
  char c;
  FILE *file;
  file = fopen("tabs.data", "r"); // Open file for reading
  if (file == NULL) {
    printf("Error opening tabs.data file.\n");
    return (1);
  }
  FILE *new_file;
  new_file = fopen("tabs_spaces.data", "w"); // Open file for reading
  if (new_file == NULL) {
    fclose(file);
    printf("Error opening tabs_spaces.data file.\n");
    return (1);
  }
  // Read tabs.data contents and convert tab to space and save in tabs_spaces
  while ((c = fgetc(file)) != EOF) {
    if (c == '\t') {
      fputc(' ', new_file);
    } else {
      fputc(c, new_file);
    }
  }

  /* exit function with no errors*/
  fclose(file);
  fclose(new_file);
  return (0);
}
/* Exercise 14_3 write a function that reads a file containing a list of
   numbers, and then writes two files. One with all the numbers divisible by 3
   and another containing all the other numbers. */

int exercise_14_3() {
  char c;
  FILE *file;
  file = fopen("numbers_list.data", "r"); // Open file for reading
  if (file == NULL) {
    printf("Error opening tabs.data file.\n");
    return (1);
  }
  /* open other_numbers.data file for writing numbers not devisable by 3 */
  FILE *new_file;
  new_file = fopen("other_numbers.data", "w"); // Open file for writing
  if (new_file == NULL) {
    fclose(file);
    printf("Error opening other_numbers.data file.\n");
    return (2);
  }
  /* open devisable file for writing for writing numbers devisable by 3 */
  FILE *devisable_file;
  devisable_file = fopen("devisable.data", "w"); // Open file for writing
  if (devisable_file == NULL) {
    fclose(new_file);
    fclose(file);
    printf("Error opening devisable.data file.\n");
    return (3);
  }
  // Read numbers_list.data contents
  char string[100];
  char *string_ptr;

  while (1) {
    string_ptr = fgets(string, sizeof(string), file);
    if (string_ptr == NULL) {
      break; // break out of the loop when end of the file
    }
    // do stuff here
    // TODO
  }

  /* close all opened files and exit function with no errors*/
  fclose(file);
  fclose(new_file);
  fclose(devisable_file);
  return (0);
}

/* Exercise 14_4 write a function that reads a ASCII file containing a list of
numbers and writes a binary file containing the sam list. Write a another
function to read in the binary to check the numbers are the same. */

void exercise_14_4() {}

/* Exercise 14_5 write a function that copies a file and removes all characters
with the high bit set (ch & 0x80)!=0) */
// open file in c
int exercise_14_5() {
  char c;
  FILE *file;
  FILE *new_file;
  /* open file */
  file = fopen("characters.data", "r"); // Open file for reading
  if (file == NULL) {
    printf("Error opening characters.data file.\n");
    return (1);
  }
  /* open file */
  new_file = fopen("characters_updated.data", "w"); // Open file for writing
  if (new_file == NULL) {
    fclose(file);
    printf("Error opening characters_updated.data file.\n");
    return (2);
  }

  // Read contents
  while ((c = fgetc(file)) != EOF) {
    if ((c & 0x80) != 0) {
      // do nothing
    } else {
      fputc(c, new_file);
    }
    /* close files */
    fclose(file);
    fputc('T', new_file);
    fclose(new_file);
  }

  return (0);
}
/* Exercise 14_6 Design a file format to store a persons name address and other
   information. write a function to read this file and produce a set of mailing
   labels.  */
void exercise_14_6() {}
