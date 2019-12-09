#!/usr/bin/env perl
use 5.028;
use warnings;

sub factors {
    my $num = shift;
    my $biggestFactor = int(sqrt($num));

    my @factors = ();
    
    for (my $i = 1; $i <= $biggestFactor; $i ++) {
        push @factors, $i if $num % $i == 0;
    }

    my $len = @factors;
    for (@factors[0..$len-1]) {
        push @factors, int($num/$_) if $num != 1;
    }

    return @factors;
}

my @factors = factors(600851475143);
@factors = sort {$a > $b} @factors;
@factors = grep {scalar(factors($_)) == 2} @factors;
say for @factors;
