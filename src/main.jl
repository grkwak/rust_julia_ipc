#=
Yes, the read function in Julia is a blocking operation. If there aren't enough bytes available in the pipe when you call read, the function will block until enough bytes are available.

In your code, read(pipe, 10) will block until 10 bytes are available to read from the pipe. If fewer than 10 bytes are available, your program will pause and wait until more bytes are written to the pipe.
=#

function call_rust_func(num::Float64)

    outgoing_pipe = open("j2r", "r+")
    incoming_pipe = open("r2j", "r+")

    # Write to the pipe
    bytes = reinterpret(UInt8, [num])
    write(outgoing_pipe, bytes) # These get_bytes do not use standard IO; instead, they use the IO stream associated with the pipe.
    flush(outgoing_pipe)
    println("JULIA: Waiting for response")

    # Wait for a certain character to be pushed to the pipe
    result = read(incoming_pipe, 8) # Blocks 10 bytes are received
    println("JULIA: Received Rust result: ", reinterpret(Float64, result)[1])
end

function main()

    println("JULIA: Begin main")
    while true
        println("JULIA: Begin loop")
        sleep(1)

        call_rust_func(30.5)
        call_rust_func(29.1)
    end
end

main()