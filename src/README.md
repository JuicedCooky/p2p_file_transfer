# main.rs
Chooses option between the user to: <br />
Start as a host and begin receiving outgoing connections
<br />
&emsp;    or 
<br />
Starting as a client which will try to connect to a specific port at a IP address

# client.rs
Similar to host.rs, just handles user input to start connection with host.
<br />
Only difference is that the client is the one to attempt the connections

# host.rs
Similar to client.rs, just handles user input to start connection with host.
<br />
Only difference is that the host is the one to receive incomming connections

# utils
This file handles file sharing options during connections and also handles the start of sending and receiving connections by both the host and client.

Read [thread files README](./thread/) for more information.
