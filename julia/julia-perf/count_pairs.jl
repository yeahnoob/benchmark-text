function groupby{T} (fn, seq::Array{T})
    dict = Dict{Any, Array{T}}()
    #set the initial-size of "dict", for better running time performance.
    sizehint(dict, length(seq) >> 5)
    for item in seq
        key = fn(item)
        if !haskey(dict, key)
            dict[key] = []
        end
        push!(dict[key], item)
    end
    return dict
end

function processdata(filename::String)
    file = open(filename, "r")
    try
        lines = String[]
        lines = readlines(file)
        word_pairs = String[]
        word_pairs = map(s->split(strip(s), ',', 2), lines)
        result = groupby(a->a[1], word_pairs)
        return result
    finally
        close(file)
    end
end

println("\n... Process [wordparis.txt] ", int(ARGS[1]), " Times ...")

for i = 1:int(ARGS[1])
    @time processdata("wordpairs.txt")
end

None 
