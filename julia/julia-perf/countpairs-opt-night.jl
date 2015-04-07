### Commentary:
### With JuliaLang v0.4-dev

function processdata!(filename::String, idx::Dict{String, Array})
    ### result store in index variable "idx"
    
    ## sort lines
    lines = sort(readlines(open(filename, "r")))
    
    ## fill the index
    firstone = 1
    lastone = 0
    prekey = ""
    for item in lines
        key = split(strip(item), ' '; limit=2)
        if key[1] == prekey
            lastone += 1
        else
            idx[prekey] = [firstone lastone]
            lastone += 1
            firstone = lastone
            prekey = key[1]
        end
    end
    return idx
end

function bench(loops::Int64, filename::String, keyword::String)
    global_idx = Dict{String, Array}()
    for i = 1:loops
        @time processdata!(filename, global_idx)
        println( "...... pairs of [", keyword, "] = ", global_idx[keyword]  )
    end
end

# bench(parse(Int64, ARGS[1]), "wordpairs.txt", "her")
None 
