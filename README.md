# kekchess

A basic chess engine in rust.

## Interactive mode

In interactive mode the program handles commands seperated by a newline from the standard input.
Interactive mode is designed to be used by a graphical frontend.

```sh
# load a fen
load <fen>

# output the current state in fen
dump

# get all possible moves for a tile (use algebraic notation)
possible_moves <tile>
# sample output:
# OK a2-a3 b1-c3 b1-a3 b2-b3

# perform a move. outputs either 'OK' or a error message
move <move>

# make the engine calculate a move and also do it if specified. outputs either 'OK <move>' or a error message
algo (random|minimax) <do move: true|false>
```

## Move serialization

```sh
# A basic move from e5 to f5
b,e5-f5

# Castling of white to king side. one of KQkq
r,K

# A pawn moved from a7 to a8 and promoted to a white queen
p,a7-a8,Q

# En passent
e,c3-d4
```
