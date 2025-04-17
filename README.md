<!-- ## ./src/main.rs
  Runs either host or client selected by user

## ./src/host.rs
  The running "server" of the first device,
  accepts all network addresses for now

## ./src/client.rs
  Connects to the server of the first device,
  connects via TCP IPv4 ip address -->

# Contributers
Alan Zheng - 100868898 <br />
Parsa Zahraei Mohammadabady - 100749173

# Download Link
[Download program executable](https://raw.githubusercontent.com/JuicedCooky/p2p_file_transfer/main/p2p.exe)


# Video Demo Link
[Link](https://drive.google.com/file/d/1gEWPYuQTtF8hmJWnRvILeDszd8c4y6zt/view?usp=sharing)

# Program
This program is a file sharing system between two devices on the same network. <br />
It makes use of multiple threads to handle multiple files being shared by creating multiple TCP connections each in its own thread sending the file and filemeta data to the corrisponding receiver.
<br /><br />
In order to make use of multiple threads we make use of the tokio library to create and run threads.

# Run instructions for Host-Client
1. Select Client or host, the host by default will receive files sent by the Client

2. The Host will start up receiving connections towards its ip address, and the Client it prompted to input the IP address with the corrosponding port that should be displayed on the Host (IP_ADDRESS:PORT)

3. Upon conformation from the Host to allow the connection, the Client will be able to select to send via a single file or send a whole folder or quit this connection to restart and/or switch roles. 

4. After selecting an option by the Client, the Host should receive a FileDialog window that prompts the user to select a path/folder to where they want to send the incoming files/folder.

5. The Client should then receive a FileDialog window to prompt the user to select a file/folder the wish to send.

6. After this the Client should have the option to send more files/folder or close the connection with the Host.

# Run instructions for Dual Host and Client

1. Select Dual session option

2. The user will be prompted to select 3 folder locations, one folder for receiving files, one folder for receiving folders, and one folder to put a log file that will record the files and folders sent to the Dual session

3. The Dual session will launch two subthreads, a host subthread that will connect to an ip address and be ready to receive files/and folders, and a client subthread that can be used to send be used to select files and folders to send in the manner described for the Client option

4. Unlike a Host or a Client, a Dual session can only make one client connection in it's client subthread and receive from 1 client in it's host subthread. Once the client subthread and host subthread connections are closed, the session will end.

# Tokio package
This runtime package for the rust programming provides:
  - Mutithreaded schedular
  - Async TCP and UDP sockets
  - Others
In particular the first two is useful for use in this program.

Basis of tokio, provides non-blocking functions

## Tokio Functions

### [tokio::main]
indicates for the main function to use tokio's runtime environment, allowing running on threads
```
#[tokio::main]
pub async fn main()
```

### tokio::spawn
```
tokio::spawn(
  async{
    *code to run*
    }); - spawns threads
```
this call automatically assigns a thread to run the lambda function or function within the parameter,
the function is required to have all the variables and values within to implement SEND type, which means it is able to safely be sent to other threads.

### task::spawn_blocking
```
tokio::spawn_blocking(
  async{
    *code to run*
    }); - spawns blocking task
```
similar to the tokio function this funtion runs code that specifically runs code that is blocking or synchronous tasks.
If a operation the file reading is done in a async the schedular stalls, however if run in spawn_blocking it will use a seperate thread pool that will not stall.


### select!
```
tokio::select! {
  _1 = something => {
    ...
  }
  _2 = something2 => {
    ...
  }
}
```
- allows current thread to switch between two or multiple contexts (something1 and something2), does not spawn threads for each context, 
- Would be like if the cpu only has one core and processor
  - Allows contexts to be switched without waiting on another context (could be due to IO events, or up to the scheduler)


## Using TcpStream across multiple threads
Since TcpStreams can't be moved across multiple threads by itself natively; std::sync::Arc and tokio::sync::Mutex datatypes need to be used.

It provides a shared ownership of a stream : TcpStream by cloning reference pointers to it on the heap.
```
let stream_read_copy = Arc::clone(&stream);

let exampleThread = tokio::spawn(async move {
  //use stream here
});
```

And to ensure that other threads that attempt to use it doesn't create a race condition and the so stream itself can be used, we require lock.

### Locks
```
stream:  Arc<Mutex<TcpStream>> <- initial lock for stream

stream_lock = stream.lock().await; <- requests to use the lock and waits if not ready
```
We can then use this to access whatever the lock contains, in this case we can access the stream once this variable is out of scope or "dropped" in rust, the lock is released.

Read [src files README](./src/) for more information.
<br />
Read [thread files README](./src/thread/) for more information.
