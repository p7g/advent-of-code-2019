function main()
  pair_regex = r"(\d)\1"
  numbers = 130254:678275

  numbers = filter(n -> occursin(pair_regex, string(n)), numbers)
  numbers = filter(n -> issorted(digits(n), rev=true), numbers)

  @info "Part 1 -> $(length(numbers))"

  numbers = filter(function(n)
    digs = digits(n)
    grouped = Dict([(i, count(isequal(i), digs)) for i in digs])
    return 2 in values(grouped)
  end, numbers)

  @info "Part 2 -> $(length(numbers))"
end

(() -> @time main())()
