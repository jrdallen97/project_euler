#!/usr/bin/env perl
use 5.028;
use warnings;

my $sum = 0;

for (my $x=1, my $y=2; $y<4000000; $y=$x+$y, $x=$y-$x) {
    $sum += $y if $y % 2 == 0;
}

say $sum;
