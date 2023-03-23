#!/usr/bin/env lua

local sum = 0

local prev, curr = 1, 2
while curr < 4000000 do
  if curr % 2 == 0 then
    sum = sum + curr
  end

  prev, curr = curr, prev + curr
end
print(sum)
