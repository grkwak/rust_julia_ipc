#=
Yes, the read function in Julia is a blocking operation. If there aren't enough bytes available in the pipe when you call read, the function will block until enough bytes are available.

In your code, read(pipe, 10) will block until 10 bytes are available to read from the pipe. If fewer than 10 bytes are available, your program will pause and wait until more bytes are written to the pipe.
=#

# func_name = { init, close}
function call_init_or_close(func_name::String, num::Float64)

    println("\n\nJULIA: Begin calling ", func_name, "()")

    outgoing_pipe = open("j2r_" * func_name, "w")

    # Write to the pipe
    bytes = reinterpret(UInt8, [num])
    write(outgoing_pipe, bytes) # These get_bytes do not use standard IO; instead, they use the IO stream associated with the pipe.
    flush(outgoing_pipe)
    close(outgoing_pipe)

    println("JULIA: Waiting for response")
    incoming_pipe = open("r2j_" * func_name, "r")
    # Wait for a certain character to be pushed to the pipe
    result = read(incoming_pipe, 8) # Blocks 10 bytes are received
    close(incoming_pipe)
    println("JULIA: Received Rust result ", reinterpret(Float64, result)[1])

end

function call_step(num::Float64, outgoing_pipe, incoming_pipe)

    println("\n\nJULIA: Begin calling step()")

    # Write to the pipe
    bytes = reinterpret(UInt8, [num])
    write(outgoing_pipe, bytes) # These get_bytes do not use standard IO; instead, they use the IO stream associated with the pipe.
    flush(outgoing_pipe)
    println("JULIA: Waiting for response")

    # Wait for a certain character to be pushed to the pipe
    result = read(incoming_pipe, 8) # Blocks until 8 bytes are received
    println("JULIA: Received Rust result ", reinterpret(Float64, result)) # First want to know that Rust finished stopping step()

    if isnan(num)
        close(outgoing_pipe)
        close(incoming_pipe)
        return
    end

end

function main()
    println("JULIA: Begin main\n\n")

    call_init_or_close("init", 1.0)

    outgoing_pipe = open("j2r_step", "w") # Open a pipe for writing
    incoming_pipe = open("r2j_step", "r") # Open a pipe for reading
    call_step(2.0, outgoing_pipe, incoming_pipe)
    call_step(3.0, outgoing_pipe, incoming_pipe)
    call_step(NaN, outgoing_pipe, incoming_pipe) # the special character that tells Rust we're done
    # Maybe when close() is called, that's when the step pipes should be closed
    call_init_or_close("close", 5.0)

    println("\n\nJULIA: End main")
end

main()