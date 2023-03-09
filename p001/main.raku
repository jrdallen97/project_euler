#!/usr/bin/env raku

my $sum = 0;

for 1..999 {
  $sum += $_ if $_ % (3 | 5) == 0
}

say $sum;
