
# Sending messages

As an example, let's send messages to the Rust program via Bash using the OpenBSD version of netcat. (The OpenBSD version seems to be the default on Ubuntu-based systems.) This should underscore the fact that the UNIX socket is really being used to communicate between two different processes.

First, compile and run the Rust program to start the socket listener:

'''bash
$ cargo run --release
```

Open up a new terminal. You should see the socket file /tmp/rust-uds.sock:

```bash
$ ls /tmp | grep rust
rust-uds.sock
```

Now let's send messages to the rust program. Use the following netcat command to open a connection to the socket.

```bash
$ nc -U /tmp/rust-uds.sock
```

The -U is necessary to indicate to netcat that this is a UNIX stream socket. 
Now, start typing text into the same window. 
Every time you press ENTER, you should see the same text appear in the terminal window in which the Rust program is running.

 Press CTRL-C to exit the Rust socket listener. 
 
 If you re-run the program, delete the old socket first: rm /tmp/rust-uds.sock
