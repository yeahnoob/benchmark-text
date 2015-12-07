### Commentary:
### With JuliaLang v0.3.3
## A Hash-Index algorithm for the text file's word-pairs counting. 
## It's seemed that Julialang's Dict data structure with AbstractString keywords 
## is slower than it with Integer keywords.

# English words' Hash Function borrowed from http://www.cse.yorku.ca/~oz/hash.html
function hashdjb2(t::AbstractString)
    ### used for english words' hash

    ## ha = convert(UINT,5381)
    ha = 0x0000000000001505
    for c in t
        ha = (ha<<5 + ha)>>3 + UInt(c)
    end
    ha
end

function processdata!( filename::AbstractString, idx::Dict{UInt, Array{UInt}}, myhash::Function = hashdjb2 )
    ### RESULT STORE IN INDEX VARIABLE "IDX"
    
    ## SORT LINES
    w = AbstractString[]
    w = readdlm(filename, ' ', AbstractString)
    const linescount = size(w,1)

    ## GET A INDEX, [ HASHVALUEOFKEYWORD ---> [MATCHEDLINESNUMBER, ...] ]
    for i in 1:linescount
        key = myhash(w[i,1])
        if haskey(idx,key)
            push!(idx[key], i)
        else
            idx[key] = [i]
        end
    end
end

function bench( loops::UInt, filename::AbstractString, keyword::AbstractString, mybenchhash::Function = hashdjb2 )
    ### benchmark processdata() function
    println("\n... Process [", filename, "] and count pairs with [", keyword, "] for [", loops, "] Times ...")
    println("... hash words with function [", string(mybenchhash), "]")
    global_idx = Dict{UInt, Array{UInt}}()
    sizehint!(global_idx, typemax(UInt16))
    for i = 1:loops
        @time processdata!(filename, global_idx, mybenchhash)
        println( "...... pairs of [", keyword, "] = ", length(global_idx[mybenchhash(keyword)]) )
    end
end

bench( parse(UInt, ARGS[1]), "wordpairs.txt", string(ARGS[2]) )
## Around 10% slower if using Julia's default hash() function
#bench( int(ARGS[1]), "wordpairs.txt", string(ARGS[2]); mybenchhash = hash )

# No More
