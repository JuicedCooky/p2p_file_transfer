## ./src/main.rs
  Runs either host or client selected by user

## ./src/host.rs
  The running "server" of the first device,
  accepts all network addresses for now

## ./src/client.rs
  Connects to the server of the first device,
  connects via TCP IPv4 ip address


# Tokio package
This runtime package for the rust programming provides:
  - Mutithreaded schedular
  - Async TCP and UDP sockets
  - Others
In particular the first two is useful for use in this program.

Basis of tokio, provides non-blocking functions

## Tokio Functions

### [tokio::main] - indicates to have the main function to use tokio's runtime

### spawn
```
tokio::spawn(*function to run*) - spawns threads
```

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


### Using TcpStream across multiple threads
Since TcpStreams can't be moved across multiple threads by itself natively; std::sync::Arc needs to be used.

It provides a shared ownership of a stream : TcpStream by cloning reference pointers to it on the heap
```
let stream_read_copy = Arc::clone(&stream);

let exampleThread = tokio::spawn(async move {
  //use stream here
});
```

And to ensure that other threads that attempt to use it doesn't create a race condition and the stream itself can be used, we require locks.

#### locks
```
stream:  Arc<Mutex<TcpStream>> <- initial lock for stream

stream_lock = stream.lock().await; <- requests to use the lock and waits if not ready
// we can then use this to access whatever the lock contains, in this case we can access the stream
// once this variable is out of scope or "dropped" in rust, the lock is released.

```

### 