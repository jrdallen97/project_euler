#!/usr/bin/env perl
use 5.028;

my @check_factors = reverse 11..19;

sub factors {
    my $num = shift;

    for (@check_factors) {
        return 0 if $num % $_ != 0;
    }

    return 1;
}

my $i = 220;
while (1) {
    last if (factors($i) == 1);

    $i += 220;
}

say $i;
