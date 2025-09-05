# hermanha-chess
Chess API built in rust

## Method?
using square centric method for keeping track of piece positions and board status. (maybe not i dont even now anymore. wellwell)

### How it works now.

board keeps track of squares and move turn. squares is 8x8 array, with either None or Piece in it. Piece has attributes color and pieceType and a method to check if a certain move shape is possible for a specific piece. Board has methods new() -> creates a new instance of itself, get(Position) -> collects whats on certain position, set(position, piece) -> sets a piece (or None) on given position, setup standard -> sets up the start position for all pieces.

in file move_piece we create more methods for Board (this seems a bit ugly will probably change), which handles moving a piece. 

method move_piece takes from position and to position. uses other methods to check if that move is legal, and if it is moves the piece in the array.

With that said. for now it works like this:
  You have to input both from move and to move. We then check a lot of things (whos turn, what piece, path between the positions etc) and moves the piece if legal. This might be dumb in the future as you have to insert both from and to position right away, although some rules just require from pos to tell illegality (for example trying to move a black piece on whites turn, it could tell you its illegal before you specify its destination)

moves that are not yet counted in are:
  en passant,
  pawn capture,
  pinned moves,
  king in check moves,
  pawns promoting,
  castling,
  might have forgot something wellwell

Oh and ofcourse, code will be refactored later. its pretty ugly, some beautiful tests maybe:o
