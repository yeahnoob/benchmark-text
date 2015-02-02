use v6;

my %dict;

grammar WordPairs {
    token word-pair { (\S*) ' ' (\S*) "\n" }
    token TOP { <word-pair>* }
}

class WordPairsActions {
    method word-pair($/) { %dict{$0}.push($1) }
}

my $match = WordPairs.parse("{@*ARGS[0]}".IO.slurp, :actions(WordPairsActions));
say ?$match;
 
say "The pairs count of the key word \"her\" in wordpairs.txt is {%dict{"her"}.elems}";

# vim: filetype=perl6
