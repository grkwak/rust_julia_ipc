function main()
    pipe = open("~/Desktop/rust_julia_ipc/", "r+")
    println("Begin")
    while true
        sleep(1)

        println(pipe, "get_bytes1") # These get_bytess do not use standard IO; instead, they use the IO stream associated with the pipe.
        flush(pipe)
        result = readline(pipe) # Blocks until a \n is received
        println("Received get_bytes_1: ", result)

        println(pipe, "get_bytes2") # These get_bytess do not use standard IO; instead, they use the IO stream associated with the pipe.
        flush(pipe)
        result = readline(pipe) # Blocks until a \n is received
        println("Received get_bytes_2: ", result)
    end
end

main()