#!/usr/bin/env perl
use 5.028;

sub factors {
    my $num = shift;

    for (reverse 11..20) {
        return 0 if $num % $_ != 0;
    }

    return 1;
}

my $i = 20;
while (1) {
    last if (factors($i) == 1);

    $i += 20;
}

say $i;
