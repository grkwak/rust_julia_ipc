function call_rust_function(function_name::String)
    open(`./path_to_your_rust_program`, "w+", stdout) do io
        println(io, function_name)
        return readline(io)
    end
end

println(call_rust_function("function1"))
println(call_rust_function("function2"))