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
# a2-a3
# b1-c3
# b1-a3
# b2-b3

# perform a move. outputs either 'OK' or a error message
move <move>

# make the engine perform a move. outputs either 'OK <move>' or a error message
move_algo (random|minimax)


```