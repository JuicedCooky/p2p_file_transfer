# write.rs

## File Writing
```
write_a_file_to_stream(stream: Arc<Mutex<TcpStream>>, path: Option<PathBuf>, print_file: bool)
```
Using the TcpStream connected to the host passed as the stream parameter, this function will read a file from the path passed in the path parameter, and write the file metadata and contents to the stream. 
<br />
<br />
The contents of the file are written to the stream in the following format: <br />
FILE <br />
FILENAME: &emsp; *name of file.type* <br />
FILESIZE: &emsp; *size of file in bytes* <br />
/n/n &emsp; *#newlines to seperate between metadata and data* <br />
DATA &emsp; *#data of the file* <br />

## Folder Writing
```
#[async_recursion]
write_a_folder_to_stream(stream: Arc<Mutex<TcpStream>>, folder_path: Option<PathBuf>, parent_folder: Option<String>)
```
This function selects a folder using FileDialog and parses each file/folder within. For each file within the folder, it will create as many threads that will individually establish a connection with the host. **The recursive function call does not separate create threads for each folder, only for each file within a folder.** <br />

This function is recursive, so for each folder within a folder, the function will run again but: 
<br /> folder_path will be the actual folder path of the current folder to parse,  

<br /> parent_folder will be the relative path of the current folder
<br /> Each function call adds all the files to a vector to which gets a corrsponding TCP port vector containing all the ports. The list of ports will be sent over to the client to parse and start reading from port and files.
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
This files contains the functions for parsing files and folders sent by client streams.

It has 4 functions that facilitate two purposes:

## File Reading
```
read_file_from_stream(stream: Arc<Mutex<TcpStream>>, file_save_location: PathBuf)
```
This parses a single file from the stream, and it is expected to receive only a single file.<br />
It takes a file_save_location argument in which the contents of the received file will be written.
<br /> 
<br /> 
From the connection it expects to receive the filename, filesize, and data of the file from the port.
Once the correct amount of data is received from the port, the function ends.

## Folder Reading
```
read_folder_from_stream(stream: Arc<Mutex<TcpStream>>, outgoing_adder:String, folder_save_location: PathBuf)
```
This reads from the main stream and determines whether it is being sent only a file or folder. If it is a file, it parses the file using the first function. 
<br />
Otherwise if it is a folder it receives, it creates threads for each folder and port read and uses the function below to assign the correct folder path and reads the contents of the connected port. 

```
parse_file_per_port_stream(address: String, folder_path: String)
```
This function connects to the corresponding port and passes the file and folder path to the file reader function, however it uses the read_file_from_stream_direct function below instead of the read_file_from_stream function to parse files, because reading the latter makes use of a mutex-locked BuffReader to read files which is not thread safe.

```
read_file_from_stream_direct(mut stream: TcpStream, file_save_location: PathBuf)
```
Instead of passing a Mutex struct that is locked by non-thread-safe datatypes, this function instead uses the Tcpstream directly since it is not expected that the stream will be passed to other threads, and so a BuffReader can safely be created to parse the file data.

## Dual Session Functions 
Seperate versions of each read function exist to be used by Dual sessions. The primary difference is that these functions, such as 

```
read_file_from_stream_dual(stream: Arc<Mutex<TcpStream>>, file_save_location: PathBuf, log_path: PathBuf)
```
take an argument log_path, which is a path the function uses to write a log text file the progress that has been made in the parsing of received files and folders. This is done in lieu of printing such information to the command-line, which would risk cluttering a Dual sessions users interface as they concurrently run their client subthread.