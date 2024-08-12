# Scopa


## Notation

Let's notate the table like this: `A B C D E ...` (where each letter represents a card) and a player's deck as `a b c d e ...`.


Possible moves (letters represent numbers):
- `a;B`: `a`'s value is equal to `B`'s value and they both get added to their pile
- `a;B+C`: `a`'s value is `B`'s + `C`'s
- `a;B+C+D`: same as above, no limit
- `a;`: `a` is an ace, player gets all cards (we're playing by `asso pigllia tutto`)
- `ta;`: place down card at index `a` on `t`able (that's why it's a `t`)


## Rules
TODO (boring to type out)


## Win condition
Points are awarded for:
- Carte: having the most cards
- Denari: having the most golden cards
- Sette bello: having the 7 di denari
- Rebello: having the re di denari
- Napoli: having a uninterrupted scale that starts from 1 of Denari. {1, 2, 3} means 1 point, {1, 2, 3, n} means n points (if you get all denari cards, you automatically win the entire game)
- Primiera: 7s thing (TODO, explain better)

there are more but BORING TO TYPEEE (TODO)

Whoever gets to 21 total points first, wins


# Turns
There are two players, Purple and Green. They start being First and Shuffler (Purple moves first). On the second match, they switch and keep switching every match.
