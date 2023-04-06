#!/usr/bin/env python3

def is_palin(n: int) -> bool:
    s = str(n)
    reversed = s[::-1]
    return s == reversed

def run(start: int, stop: int) -> None:
    found: list[int] = []
    for x in range(start, stop):
        for y in range(start, stop):
            n = x * y
            if is_palin(n):
                found.append(n)
    print("Largest palindrome:", max(found))

run(100, 999)
