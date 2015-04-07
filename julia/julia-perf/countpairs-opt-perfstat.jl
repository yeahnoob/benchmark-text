### Commentary:
### With JuliaLang v0.3.3
## A Hash-Index algorithm for the text file's word-pairs counting. 
## It's seemed that Julialang's Dict data structure with String keywords 
## is slower than it with Integer keywords.

# English words' Hash Function borrowed from http://www.cse.yorku.ca/~oz/hash.html
function hashdjb2(t::String)
    ### ha = convert(Uint,5381)
    ha = 0x0000000000001505
    for c in t
        ha = (ha<<5 + ha)>>3 + Int(c)
    end
    ha
end

function processdata!( filename::String, idx::Dict{Uint, Array{Uint}}; myhash::Function = hashdjb2 )
    ## result store in index variable "idx"
    
    ## sort lines
    W = String[]
    W = readdlm(filename, ' ', String)
    const linescount = size(W,1)

    ## get a index, [ HashValueOfKeyword ---> [MatchedLinesNumber, ...] ]
    for i in 1:linescount
        key = myhash(W[i,1])
        if haskey(idx,key)
            push!(idx[key], i)
        else
            idx[key] = [i]
        end
    end
end

# benchmark processdata() function
#=
global_idx = Dict{Uint, Array{Uint}}()
sizehint(global_idx, typemax(Uint16))
processdata!("wordpairs.txt", global_idx)
=#

# Around 10% slower if using Julia's default hash() function
None
