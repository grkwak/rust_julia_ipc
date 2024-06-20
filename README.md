# Rust-Julia Inter-Process Communication (IPC) Demo
## Description
- Want to have a process always running, blocking read, not over-using CPU resources
- Not use standard IO because it's slow and a lot of stuff is already using it
- Want to send raw bytes, not string-ified
- Re-building executable is fine, and not having to restart Julia is fine
## Design considerations
### Pipes
Pipes are one of the **simplest forms of IPC** and are typically used for communication between parent and child processes or between two closely related processes. They allow one process to write data to the pipe, which can then be read by another process. Pipes are unidirectional by default, but can be made bidirectional. They are **best for small amounts of data and are not suitable for complex data structures**.
- Once data is read from a pipe, it is removed from the pipe and cannot be read again.
- Waiting until there is nothing left to read from a pipe can be tricky because pipes are designed for continuous data flow and don't have a built-in way to signal when no more data will be sent. However, you can design your own protocol for signaling the end of data.
### Shared memory
Shared memory is a method of IPC where multiple processes can access and modify the same region of memory. It's one of the fastest methods of IPC because it allows direct access to memory, but it's also one of the most complex because it requires **careful synchronization to avoid race conditions**. Shared memory is **best for large amounts of data or complex data structures**.
### Sockets
Sockets are used for communication between processes on the same machine or on different machines. They are **more complex than pipes or shared memory** because they involve networking concepts like ports and IP addresses. UDP (User Datagram Protocol) sockets are connectionless and do not guarantee delivery or order of data, making them faster but less reliable. TCP (Transmission Control Protocol) sockets are connection-oriented and guarantee delivery and order of data, making them slower but more reliable. Sockets are **best for communication over a network, especially between different machines**.