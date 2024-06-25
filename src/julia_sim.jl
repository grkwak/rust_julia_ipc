#=
Yes, the read function in Julia is a blocking operation. If there aren't enough bytes available in the pipe when you call read, the function will block until enough bytes are available.

In your code, read(pipe, 10) will block until 10 bytes are available to read from the pipe. If fewer than 10 bytes are available, your program will pause and wait until more bytes are written to the pipe.
=#

# func_name = { init, step, close}
function call_rust_func(func_name::String, num::Float64)

    println("\n\nJULIA: Begin call_rust_func")

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

function main()
    println("JULIA: Begin main\n\n")

    call_rust_func("init", 1.0)
    call_rust_func("step", 2.0)
    call_rust_func("step", 3.0)
    call_rust_func("step", 999) # the special character that tells Rust we're done
    call_rust_func("close", 5.0)

    println("\n\nJULIA: End main")
end

main()