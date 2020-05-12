#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <unistd.h>

// function to find a prime number
unsigned is_prime(unsigned n) { 
  if (n <= 3) return n > 1;
  if (0 == (n % 2) || 0 == (n % 3)) return 0;

  unsigned i;
  for (i = 5; (i * i) <= n; i += 6)
    if (0 == (n % i) || 0 == (n % (i + 2))) return 0;

  return 1; // found a prime! 
}

int main() {
  const char* file = "./fifoChannel";  // file name of the named pipe
  int fd = open(file, O_RDONLY);      // open the pipe as read only
  if (fd < 0) exit(-1);              // if pipe not found exit the program
  unsigned count = 0, total = 0, primes_count = 0;

// ------------ read in data from the named pipe -----------------------
  while (1) {
    int next;
    int i;

    ssize_t count = read(fd, &next, sizeof(int)); // read from named pipe
    if (0 == count) break;                  // end of stream break while loop
    else if (count == sizeof(int)) {        // read a 4-byte int value 
      total++;
      if (is_prime(next)) primes_count++;
    }
  }
// ----------- close named pipe -------------------------------------------------
  close(fd);       // close pipe from read end 
  unlink(file);    // unlink from the underlying file 
  // output data from the named pipe
  printf("Received ints: %u, primes: %u\n", total, primes_count);

  exit(0); // exit normally
}