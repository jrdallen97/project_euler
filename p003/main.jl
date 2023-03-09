#!/usr/bin/env julia

function factors(num)
  # The highest unique factor is sqrt(num).
  # After that we'll just be finding the matching "pair" to all the factors we've already found.
  highest::Int64 = floor(sqrt(num))
  found::Vector{Int64} = []

  # Handle the sqrt case to avoid double-counting
  if num % highest == 0
    append!(found, highest)
  end

  for i in 1:highest-1
    if num % i == 0
      append!(found, i, num/i)
    end
  end
  return sort(found)
end

function prime(num)
  return length(factors(num)) == 2
end

function main()
  println(maximum(filter(prime, factors(600851475143))))
end

@time main()
