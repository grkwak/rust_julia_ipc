#=
Yes, the read function in Julia is a blocking operation. If there aren't enough bytes available in the pipe when you call read, the function will block until enough bytes are available.

In your code, read(pipe, 10) will block until 10 bytes are available to read from the pipe. If fewer than 10 bytes are available, your program will pause and wait until more bytes are written to the pipe.
=#

function main()
    outgoing_pipe = open("j2r", "r+")
    incoming_pipe = open("r2j", "r+")

    println("JULIA: Begin main")
    for i in 1:5
        println("JULIA: Begin loop ", i)
        sleep(1)

        # Write to the pipe
        print(outgoing_pipe, "1") # These get_bytes do not use standard IO; instead, they use the IO stream associated with the pipe.
        flush(outgoing_pipe)
        println("JULIA: Waiting for response")

        # Wait for a certain character to be pushed to the pipe
        result = read(incoming_pipe, 5) # Blocks 10 bytes are received
        println("JULIA: Received get_bytes_1: ", result)

        # Write to the pipe
        print(outgoing_pipe, "2") # These get_bytes do not use standard IO; instead, they use the IO stream associated with the pipe.
        flush(outgoing_pipe)
        println("JULIA: Waiting for response")

        # Wait for a certain character to be pushed to the pipe
        result = read(incoming_pipe, 5) # Blocks until 10 bytes are received
        println("JULIA: Received get_bytes_2: ", result)
    end
end

main()