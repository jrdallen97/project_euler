#!/usr/bin/env julia

function main()
  sum = 0
  prev, curr = 1, 2
  while curr < 4000000
    if curr % 2 == 0
      sum += curr
    end

    prev, curr = curr, prev + curr
  end

  println(sum)
end

main()
