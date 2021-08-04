# Bitboard representation

A bitboard is an integer representing a board. In regular chess a board has a dimension of `8 * 8 = 64` which is, when we think about it, kind of a big coincidence with the fact that modern computer are based of 64 bits processors. In a bitboard integer, when a bit is set at the index `n` it means that a piece occupies this square.
In the following board an upper letter represents a white piece and a lower letter a black piece, next to it you can see a bitboard representing its occupancy (meaning that we can only retrieve information about wether a square is occupied or not but not by which piece).
```
 ┏━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┓
8┃   ┃   ┃   ┃   ┃   ┃   ┃   ┃   ┃
 ┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫
7┃ R ┃   ┃   ┃   ┃   ┃ p ┃ k ┃   ┃
 ┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫ 
6┃   ┃   ┃   ┃   ┃   ┃ p ┃   ┃   ┃
 ┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫
5┃   ┃ p ┃   ┃ r ┃   ┃   ┃   ┃ p ┃
 ┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫
4┃   ┃   ┃   ┃   ┃   ┃   ┃   ┃   ┃  8 00000000
 ┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫  7 10000109
3┃ P ┃   ┃   ┃   ┃   ┃ K ┃ P ┃   ┃  6 00000100
 ┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫  5 01010001
2┃   ┃   ┃   ┃   ┃   ┃ P ┃   ┃ P ┃  4 00000000
 ┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫  3 10000110
1┃   ┃   ┃   ┃   ┃   ┃   ┃   ┃   ┃  2 00000101
 ┗━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┛  1 00000000
   a   b   c   d   e   f   g   h      abcdefgh
```

```
                            00000000<--End of the integer
                            10000110
                            00000100
                            01010001
                            00000000
                            10000110    
                            00000101    
Begginging of the integer-->00000000    
```

To store all the information about a board we acutally need multiple bitboards, one for each piece type/color.

In the case of TicTacChess we doesn't need a full 64 integer to represent a 4x4 board. Our board has 16 squares but we cannot use a 16 bits integer, indeed when computing if four pieces are aligned we use bitshifting and AND operator like so:
```
0000   0000   0000   0000   0000
1111   0111   0011   0001   0001
0000 & 1000 & 1100 & 1110 = 0000
0000   0000   0000   0000   0000
```
In this case we don't have any problems, but let's analyse the following board:
```
0001   0000   0000   0000   0000
1110   1111   0111   0011   0010
0000 & 0000 & 1000 & 1100 = 0000
0000   0000   0000   0000   0000
```
We can see in the original board is not a winning board however our algorithm will detect a win. To overcome this issues we had a "ghost" column:

```
00010   00001   00000   00000   00000
11100   01110   10111   01011   00000
00000 & 00000 & 00000 & 10000 = 00000
00000   00000   00000   00000   00000
    ^
    |
Ghost column
```

So in order to use board with a ghost column we need to take the size above 16 bits integer which is 32 bits integer, here an exemple:

```
 ┏━━━┳━━━┳━━━┳━━━┓
4┃   ┃   ┃   ┃   ┃
 ┣━━━╋━━━╋━━━╋━━━┫
3┃ P ┃   ┃ p ┃   ┃
 ┣━━━╋━━━╋━━━╋━━━┫         
2┃   ┃ k ┃ R ┃   ┃  4 00000000000000000
 ┣━━━╋━━━╋━━━╋━━━┫  3 10100
1┃   ┃ B ┃   ┃   ┃  2 01100
 ┗━━━┻━━━┻━━━┻━━━┛  1 01000
   a   b   c   d      abcd
```

