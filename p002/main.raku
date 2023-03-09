#!/usr/bin/env raku

my $sum = 0;

loop (my ($x, $y) = (1, 2); $y < 4000000; ($x, $y) = ($y, $x+$y)) {
    $sum += $y if $y % 2 == 0;
}

say $sum;
