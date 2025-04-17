# main.rs
Presents the user with the options to begin the following types of sessions: 

## Host session
In this type of session, the user is connected to an available port and can receive connection requests from client sessions. If accepted, the user will read in and parse files and folders sent by the client, and save them in selected folders. Once a client ends a connection, the user will have an option to wait for another connection request.

## Client session
In a client session, the user can request connections with available host sessions. If accepted, the user can select files and folders to the send to the host session. The user can aslo break a connection with a host session and connect to a new one.

## Dual session
This section combines the functionality of the host and client sessions using subthreads. On start, a host subthread is launched and waits for a connection. Simultaneously, a client subthread is launched and allows the user to connect and send files/folders to either a host session or another dual session. A dual session can one host and client connection each, with the session terminating once both of them have been closed.

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