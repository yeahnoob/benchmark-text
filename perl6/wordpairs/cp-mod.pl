use v6;

=begin original
my $file=open "wordpairs.txt", :r;

my %dict;
my $line;

repeat {
    $line=$file.get;
    my ($p1,$p2)=$line.split(' ');
    if ?%dict{$p1} {
        %dict{$p1} = "{%dict{$p1}} {$p2}".words;
    } else {
        %dict{$p1} = $p2;
    }
} while !$file.eof;
=end original

my %dict;
#my $io="{@*ARGS[0]}".IO;
my $contents = "{@*ARGS[0]}".IO.slurp;
for $contents.words(:eager) -> $a,$b {
    push %dict{$a}, $b;
}

## Test
say "The pairs count of the key word \"her\" in wordpairs.txt is {%dict{"her"}.elems}";

# vim: ft=perl6
