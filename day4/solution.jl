module Day4

export main

const nums = [0,0,0,0,0,0,0,0,0,0]::Array{Int64,1}

function has2group(digs::Array{Int64,1})::Bool
    for i in eachindex(nums)
        nums[i] = 0
    end

    for d in digs
        nums[d] += 1
    end

    return 2 in nums
end

function main()::Tuple{Int32,Int32}
    pair_regex = r"(\d)\1"
    local range = 130254:678275

    numbers = range[occursin.(pair_regex, string.(range))]
    numbers = numbers[issorted.(digits.(numbers), rev = true)]

    part1 = length(numbers)

    numbers = numbers[has2group.(digits.(numbers))]

    part1, length(numbers)
end

end