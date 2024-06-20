#=
Yes, the read function in Julia is a blocking operation. If there aren't enough bytes available in the pipe when you call read, the function will block until enough bytes are available.

In your code, read(pipe, 10) will block until 10 bytes are available to read from the pipe. If fewer than 10 bytes are available, your program will pause and wait until more bytes are written to the pipe.
=#

function main()
    pipe = open("pipe_name", "r+")
    println("Begin")
    while true
        sleep(1)

        println(pipe, "get_bytes1") # These get_bytes do not use standard IO; instead, they use the IO stream associated with the pipe.
        flush(pipe)
        result = read(pipe, 10) # Blocks 10 bytes are received
        println("Received get_bytes_1: ", result)

        println(pipe, "get_bytes2") # These get_bytes do not use standard IO; instead, they use the IO stream associated with the pipe.
        flush(pipe)
        result = read(pipe, 10) # Blocks until 10 bytes are received
        println("Received get_bytes_2: ", result)
    end
end

main()