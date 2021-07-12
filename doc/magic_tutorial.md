# Magic Bitboard


## Table of contents

- [Introduction](#introduction)
- [Why](#why)
- [How](#how)
  - [Theory](#theory)
    - [Key space reduction](#key-space-reduction)
    - [The magic bitboard structure](#the-magic-bitboard-structure)
    - [Building the magic bitboard](#building-the-magic-bitboard)

## Introduction

During this tutorial I'll assume that you're familiar with the concept of bitboards.
Also I'll use the following representation of a chess board to illustrate my points:
```
rhbqkbhr
pppppppp
........
........
........
........
PPPPPPPP
RHBQKBHR

Description:
    r: black rook
    h: black knight
    b: black bishop
    q: black queen
    k: black king
    p: black pawn
    

    R: white rook
    H: white knight
    B: white bishop
    Q: white queen
    K: white king
    P: white pawn

    .: empty cell
```

And when I'll talk about bitboard more specifically I'll use the following representation
```
  end of the integer --> 11111111
                         11111111
                         ........
                         ........
                         ........
                         ........
                         11111111
start of the integer --> 11111111

where . corresponds to a 0 in the 64-bits integer
```

This first part explains the motivation behind magic bitboards, if you're only here for a tutorial you can skip to the [next part](#how).


## Why

One feature of a chess engine is to know where a piece can move in function of the board state. One category of pieces is called "sliding piece", this category includes the rooks, the bishops and the queen. All those pieces have the particularity that they can slide on board "indefinitely" until another piece or the border of the board blocks it.  

This characteristic makes their possible moves hard to generate. But thanks to some passionate peoples several techniques have been created that allow for fast computation of the possible moves of a sliding piece given its type, position and board state. In this tutorial we'll focus mainly on the technique called "magic bitboard".

## How

### Theory

#### Key space reduction

The idea behind magic bitboard is to create a map for each combination of piece type and position that contains all the possible moves for all the combination of board state. You can see a magic bitboard as a function that takes as input the piece type/position and the board state and returns the legal move for this piece.

```
magic(piece_type, position, board_state) = legal_moves
```

Or more visually:

```
              ........  ........    ........
              ........  ........    1.......
              ........  .....1..    .1...1..
              ........  ........    ..1.1...
magic(bishop, ...1...., ........) = ........
              ........  ........    ..1.1...
              ........  ........    .1...1..
              ........  ........    1.....1.

Note: we don't care about the piece color when using magic bitboard.
Indeed if the other piece on the board is of the same color we only
need to remove the same piece color from the legal_moves (using bit operation).

In this example we would obtain:

........
1.......
.1......
..1.1...
........
..1.1...
.1...1..
1.....1.

```

As you can expect we can not store all the possible combination of piece type, position and board state in the keys of map, there are way to many of them. There are two observations we can make that'll let us reduce the key space drastically:

* We only need to take into account pieces that are on the trajectory (also called attacks) of the piece
* Some combination of piece type, position and board state have the same legal moves (see example bellow)


```
              ........  ........    ........
              ........  ........    1.......
              ........  .....1..    .1...1..
              ........  ........    ..1.1...
magic(bishop, ...1...., ........) = ........
              ........  ........    ..1.1...
              ........  ........    .1...1..
              ........  ........    1.....1.
              

              ........  ........    ........
              ........  .....1..    1.......
              ........  ....1...    .1...1..
              ........  ........    ..1.1...
magic(bishop, ...1...., ........) = ........
              ........  ........    ..1.1...
              ........  ........    .1...1..
              ........  ........    1.....1.
              

              ........  .......1    ........
              ........  ........    1.......
              ........  .....1..    .1...1..
              ........  ........    ..1.1...
magic(bishop, ...1...., ........) = ........
              ........  ........    ..1.1...
              ........  ........    .1...1..
              ........  ........    1.....1.
```

In order to use the first observation at our advantage we use [bit masking](https://en.wikipedia.org/wiki/Mask_(computing)) to get only the relevant pieces of board.

For example let's take the following board with a bishop on d4 (`1` denotes a piece, we don't care which one in this example):

```
8 ........
7 ........
6 .....1..
5 ...1....
4 .1.b....
3 ........
2 ...1...1
1 .1....11
  abcdefgh
```

On this board the only relevant pieces in order to get the bishops moves are the ones on its diagonals, i.e:

```
8 .......1 
7 1.....1. 
6 .1...1.. 
5 ..1.1... 
4 ........ 
3 ..1.1... 
2 .1...1.. 
1 1.....1. 
  abcdefgh
```

To get the board with only the relevant pieces we use the bitwise `and` operator (also denoted `&` in C/C++)


```
 .......1     ........   ........
 1.....1.     ........   ........
 .1...1..     .....1..   .....1..
 ..1.1...     ...1....   ........
 ........ and .1.b.... = ........
 ..1.1...     ........   ........
 .1...1..     ...1...1   ........
 1.....1.     .1....11   ......1.
```

Another observation we can make is that a piece that is on the border of the board doesn't have any impact on the possibles moves.

```
              ........  ........    ........
              ........  ........    1.......
              ........  .....1..    .1...1..
              ........  ........    ..1.1...
magic(bishop, ...1...., ........) = ........
              ........  ........    ..1.1...
              ........  ........    .1...1..
              ........  ........    1.....1.
              

              ........  ........    ........
              ........  ........    1.......
              ........  .....1..    .1...1..
              ........  ........    ..1.1...
magic(bishop, ...1...., ........) = ........
              ........  ........    ..1.1...
              ........  ........    .1...1..
              ........  ......1.    1.....1.
```

So we only need a mask that stop at one square before the borders.

```
 ........     ........   ........
 1.....1.     ........   ........
 .1...1..     .....1..   .....1..
 ..1.1...     ...1....   ........
 ........ and .1.b.... = ........
 ..1.1...     ........   ........
 .1...1..     ...1...1   ........
 ........     .1....11   ........
```

Note that for the rook this property is only applied to the end of its rays.


```
 ........     1..1....   ........
 1.......     ........   ........
 1.......     .....1..   ........
 1.......     ........   ........
 .111111. and r...1... = ....1...
 1.......     ........   ........
 1.......     1..1...1   1.......
 ........     .1....11   ........
```


#### The magic bitboard structure

We talked a lot about reducing the key space of our map, now we'll talk about the details of how we use a magic bitboard.
One magic bitboard corresponds to one piece at one position. A magic bitboard is composed of:

* **A magic number**: which is a 64-bits integer
* **A database** (array) containing all the possible move for a specific piece at a specific position.
* **The shift** which is a integer inferior to 64

With those three components we can retrieve the moves for a piece thanks to the following formula (using C++ notation):

```
moves = database[((board & mask) * magic) >> (64 - shift)]
```

A lot is going in this formula so let's break it down.

* `(board & mask)`: this part allows to retrieve only the relevant pieces from the board as explained in the previous part
* `(board & mask) * magic`: here is the tricky part, we multiply our board by some magic constant (constant by piece/position) in order to get the index of our array on the "top" of our integer (don't worry if you don't understand for the moment, an example is coming)
* `(board & mask) * magic >> (64 - shift)`: The index in the `database` array of the move is now on the top of `(board & mask) * magic`, so we need to bring it down in order to get its value, the value of shift depends on the quality of the *magic number*
* `database[((board & mask) * magic) >> (64 - shift)]`: now that we have our index we only need to retrieve the corresponding moves in the database array


Let's take our bishop in d4:
```
8 ........
7 ........
6 .....1..
5 ...1....
4 .1.b....
3 ........
2 ...1...1
1 .1....11
  abcdefgh
```
    
First we use a bit mask to get the relevant pieces:

```
 ........     ........   ........
 ........     1.....1.   ........
 .....1..     .1...1..   .....1..
 ...1....     ..1.1...   ........
 .1.b.... and ........ = ........
 ........     ..1.1...   ........
 ...1...1     .1...1..   ........
 .1....11     ........   ........
```

Now we multiply it by the magic number (here we don't specify it, we'll explain how to find it later) which gives use this imaginary result:

```
........           11.1....
........           ........
.....1..           ........
........           ........
........ * magic = ........
........           ........
........           ........
........           ........
```

Now let's imagine that we have `shift = 4`. The third step gives us the following result:

```
11.1....         ........
........         ........
........         ........
........         ........
........ >> 60 = ........ = 13
........         ........
........         ........
........         ....11.1
```

Then in the `database` array at the index `13` we have:

```
               ........
               1.......
               .1...1..
               ..1.1...
database[13] = ........
               ..1.1...
               .1...1..
               1.....1.
```

Which corresponds to the possible moves for our bishop in d4.

#### Building the magic bitboard

Pseudo code of the algorithm used to find magic bitboard:

![](./images/find_magic)

Again there's a lot going on here so let's break it down.   
First we find all the possible setup of blockers of a the given piece at ts given position. Then we need to know on how many bits the index given after multiplying by the magic and shifting will coded, this is given by `log2(number of blockers possition) + 1`, note that `log2` gives a float output, in order to be sure that we have enough place for all the possible moves of blockers we add 1 to this value and then remove the decimal part (ex: `log2(5) = 2.3219` so the `bits = 3` and we're sure to have enough place for all the moves). We then initialize `database` and `magic` to default value. `failed` is a boolean that is set to `true` if the magic currently tested is not valid.

In the loop part we test all the possible block boards and check if the current `magic` is valid. A `magic` number is valid for a given block board if at the iteration we test the block board in the `for` loop the `database` array at the index `board * magic >> (64 - bits)` is either 0 or the corresponding move board.

Then if the current `magic` is not valid we change it to another random value and clear the `database`.

This algorithm doesn't look very smart because we test magic number randomly but finding a valid magic is fast and only done once at the start of the engine.

## Implementation

TODO
