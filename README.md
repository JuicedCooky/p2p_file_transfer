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
