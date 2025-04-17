# main.rs
Presents the user with the options to begin the following types of sessions: 

## Host session
In this type of session, the user is connected to an available port and can receive connection requests from client sessions. If accepted, the user will read in and parse files and folders sent by the client, and save them in selected folders. Once a client ends a connection, the user will have an option to wait for another connection request.

## Client session
In a client session, the user can request a connection with an available host session. If accepted, the user can select files and folders to the send to the host session. The user can aslo break a connection with a host session and connect to a new one.

## Dual session
This section combines the functionality of the host and client sessions using subthreads. On start, a host subthread is launched and waits for a connection. Simultaneously, a client subthread is launched and allows the user to connect and send files/folders to either a host session or another dual session. A dual session can make only one host and client connection each, with the session terminating once both of them have been closed.

# host.rs
Defines a struct Host that facilitates the logic of file and folder reception. The struct method new initializes a stream on a local port and waits for oncoming connections from a client session. Once a connection is established, the helper function hand_host_session is called, which parses a message from the client that tells the host what type of content to expect, and invokes the appropriate file or folder reading function from read.rs

# client.rs
Similar to host.rs, this file defines a struct Client that facilitates the logic of file and folder sending. The struct method new prompts the user to enter the ip-address and port number of a host stream. Once a connection is established, the function display_options from utils.rs is called. 

# utils.s
In conjuction with client.rs, this file contains functions to facilitate file and folder sending. The afforementioned display_options function presents the user with the option to send a file or a folder to the host, and also to break the connection with the host. Based on the user's selection, the helper function handle_sender_session is called, which sends a message to host on what type of content to expect and invokes the appropriate file or folder writing function from write.rs

# dual.rs
Defines a struct Dual, which combines the functionality of Host and Client through the use of tokio threads. The struct method new prompts the user to select folder locations to receive files, folders, and a to place a text log file that records the details of received content. It then creates two tokio threads, one which invokes the struct method host_sub_session, and another which invokes the struct method client_sub_session. The struct method host_sub_session has similar functionality to the struct methods and functions contained in host.rs, allowing the dual session user to receive content from client sessions or another dual sessions' client subsession. Concurrently, the struct method client_sub_session performs similar the struct methods and functions contained within clien.rs and utils.rs, allowing the dual user to send data to a host session or another dual sessions host subsession.

Read [thread files README](./thread/) for more information.
