#!/usr/bin/env perl
use 5.028;

sub isPalin {
    my $num = shift;
    while (length $num > 1) {
        $num =~ /^(.)(.*)(.)$/;
        return 0 unless $1 eq $3;
        $num = $2;
    }
    return 1;
}

my @nums = ();
for my $x (1..999) {
    for my $y (1..999) {
        push @nums, $x*$y if isPalin($x*$y);
    }
}

@nums = sort {$a <=> $b} @nums;
say for $nums[-1];
