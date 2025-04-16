# write.rs
```
write_a_file_to_stream(stream: Arc<Mutex<TcpStream>>, path: Option<PathBuf>, print_file: bool)
```
Using the connected TcpStream in the stream parameter, this function will read a file using the corresponding path in the parameter, and write the file metadata and contents to the stream. 
<br />
<br />
The contents are formatted here: <br />
FILE <br />
FILENAME: &emsp; *name of file.type* <br />
FILESIZE: &emsp; *size of file in bytes* <br />
/n/n &emsp; *#newlines to seperate between metadata and data* <br />
DATA &emsp; *#data of the file* <br />

```
#[async_recursion]
write_a_folder_to_stream(stream: Arc<Mutex<TcpStream>>, folder_path: Option<PathBuf>, parent_folder: Option<String>)
```

## Description
This function selects a folder using FileDialog and parses each file/folder within, for each file within the folder, it will create that many threads that individually startup a connection with the host. **The recursive function call does not create threads for each folder, only for each file within a folder.** <br />


This function is recursive, so for each folder within a folder, the function will run again but 
<br /> folder_path will be the actual folder path of the current folder to parse,  

<br /> parent_folder will be the relative path of the current folder
<br /> Each function call adds all the files to a vector to which gets a corrsponding TCP port vector containing all the ports. The list of ports will be sent over to the receiver to parse and start reading from port and files.
<br />
<br />
The content are formatted like this:
<br />
FOLDER *indicates start of folder contents* <br />
FOLDER: *the relative path of the folder* <br />
PORT: *port number #1* <br />
PORT: *port number #...* <br />
PORT: *port number #N* <br />
<br />
*if the folder is a sub folder* <br />
END FOLDER *indicates to stop writing to the corrosponding relative folder*<br />
<br />
*if the folder is the root folder* <br />
END *indicates to stop expecting FOLDER: and PORT: to parse*<br />

### Macro
Since this function is asynchronous and recurisve, we would need to specify the fixed size of the return type using Box::pin but here we just use the macro: #[async_recursion] to hide the return type.

# read.rs
This files handles all the receiving of the files.

It has 4 functions:
```
read_file_from_stream(stream: Arc<Mutex<TcpStream>>, file_save_location: PathBuf)
```
This parses a single file from the stream, it is expected to receive a single file.<br />
It expects the file_location to write into which is in the parameter.
<br /> 
<br /> 
From the connection it expects the filename, filesize, and data of the file from the port.
Once the correct amount of data is received from the port, the function ends.

```
read_folder_from_stream(stream: Arc<Mutex<TcpStream>>, outgoing_adder:String, folder_save_location: PathBuf)
```
This reads from the main stream and determines whether it is being sent only a file or folder. If it is a file, it parses the file using the first function. 
<br />
Otherwise if it is a folder it receives, it creates threads for each folder and port read and uses the function below to assign the correct folder path and reads the contents of the connected port. 

```
parse_file_per_port_stream(address: String, folder_path: String)
```
This just connects to the corresponding port and starts paring the file, however it uses the function below instead of the first function to parse files because reading the files recuire the use of BuffReader which is not thread safe, and so trying to run the thread in the first function that acquires the lock of the TcpStream to be used in BuffReader and for see below why it works for that function.

```
read_file_from_stream_direct(mut stream: TcpStream, file_save_location: PathBuf)
```
Instead of passing a Mutexes that is locked by non-thread-safe datatypes, it instead uses the Tcpstream directly since we do not expect to pass the stream to other threads, and so BuffReader will not cause an error since the stream will not be shared.
