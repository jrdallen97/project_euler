#!/usr/bin/env julia

function main()
  sum = 0
  for i in 1:999
    if i % 3 == 0 || i % 5 == 0
      sum += i
    end
  end
  println(sum)
end

main()
