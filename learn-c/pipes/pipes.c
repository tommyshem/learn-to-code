// Simple Example of using pipes with forked child process
// compile with 
// clang -Wall -Werror pipes.c -o pipes 

#include <sys/wait.h> // wait
#include <stdio.h>
#include <stdlib.h> // exit functions
#include <unistd.h> // read, write, pipe, exit
#include <string.h>

#define ReadEnd 0  // stdin
#define WriteEnd 1 // stdout

void report_and_exit(const char *msg)
{
	perror(msg);
	exit(-1); // failure
}

int main()
{
	int pipeFDs[2];										// two file descriptors
	char buf;											// 1 byte buffer
	const char *msg = "Natures first green is gold \n"; // bytes to write through pipe

	if (pipe(pipeFDs) < 0)			// get stdin and stdout and store them
		report_and_exit("pipeFD");  // exit if return is -1 as error opening pipes

	pid_t cpid = fork(); // fork a child process passes back pid integer number

	if (cpid < 0)				// check if process fork and created a child process -1 failed
		report_and_exit("fork"); // check for failure -1 passed if child process not started
								 // else cpid holds the pid number of the child process
								 // and the fork child process holds zero

	if (0 == cpid)
	{							  // child process == 0 so run this code in forked process
		// ---------------- child process runs this code only --------------------
		close(pipeFDs[WriteEnd]); // child reads, doesn't write so close pipe

		while (read(pipeFDs[ReadEnd], &buf, 1) > 0)	 //
			write(STDOUT_FILENO, &buf, sizeof(buf)); // echo to the standard out
		close(pipeFDs[ReadEnd]);					 // close stdin pipe
		_exit(0);									 // notify pararent at once
	}
	else
	{
		// ---------------- parent process runs this code only --------------------
		close(pipeFDs[ReadEnd]);					// parent writes, doesn't read
		write(pipeFDs[WriteEnd], msg, strlen(msg)); // write the bytes to the pipe
		close(pipeFDs[WriteEnd]);					// done writing generate end of

		wait(NULL); // wait for child process to end
		exit(0);	// exit normally
	}
	return 0;
}