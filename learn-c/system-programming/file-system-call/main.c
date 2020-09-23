// create, add text to the file or add text to an exiting file
#include <fcntl.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h> 

// compile with clang -o great ./main.c

int main(){
	int fd; // file descriptor variable
	fd = open("test-file",O_WRONLY | O_CREAT, 0644); //open or create file
	// error checking
	if (fd < 0){
		printf("error number %d \n",errno); // errno is global variable 
		perror("error"); // the error descriptions is added on the end 
		exit(1); // exit program with exit code
	}
	write(fd, "hello world\n", 12); // write to the file
	close(fd); // close the file
	return 0; // exit program
}
