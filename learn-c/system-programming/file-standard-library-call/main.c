#include <stdio.h>
#include <stdlib.h>
#include <errno.h>

int main(){
FILE *fp; //file pointer
fp=fopen("./test_file", "wb"); // open file
// modes
// r  - open for reading
// w  - open for writing (file need not exist eg create file)
// a  - open for appending (file need not exist eg create file)
// r+ - open for reading and writing, start at beginning
// w+ - open for reading and writing (overwrite file)
// a+ - open for reading and writing (append if file exists)

char x[15]="Hello there !\n"; // c string
int size_read = fwrite(x, sizeof(x[0]), sizeof(x)/sizeof(x[0]), fp); // write to the file
// error checking
        if (size_read < 0){
                printf("error number %d \n",errno); // errno is global variable 
                perror("error"); // the error descriptions is added on the end 
                exit(1); // exit program with exit code
        }
fclose (fp); // close the file
return 0; // exit program
}
