# hermanha-chess
Chess API built in rust

# How it Works!

**Board**: Board does it all basically. it keeps track of the game_state and has the logic for move legality. in board.rs you will find the base responsibilities, setting, getting, etc. in rules.rs you will find the logic for moving a piece. It might not be optional, but it works and its kind of easy to understand, for me at least.

**Rules**: 
1. check basic things, like positions on board, not same pos, etc. basic legality i guess without checking movement at all.
2. classify movetype, check if its a normal, capture, en passant, pawn promotion or castle. Logic will be little different depending on the move.
3. then we check for movement legality based on the move type. checking for things like move shape, blocking pieces etc
4. move in check, when we have established that the move is possible, we fake making the move in a cloned version of the board, and then checks if the king is in check. if it is we cant make that move (duhhh).
5. set values, depending on the move type we need to set pieces a bit differently, for example for en passant and castle. the set_values method therefore has some logic for different pieces:D
If the move is legal, it will return MoveOk (type alias for ()), if not it will return MoveError which can be one of theese:
* EmptyFrom,
* WrongTurn,
* IllegalShape,
* Blocked { at: Position },
* SelfCheck,
* SameSquare,
* OutOfBounds,
* CaptureOwn,
* KingHasMoved,
* RookHasMoved

**NOTE:** old note:)) (ill keep it for now) With that said. for now it works like this: You have to input both from move and to move. We then check a lot of things (whos turn, what piece, path between the positions etc) and moves the piece if legal. This might be dumb in the future as you have to insert both from and to position right away, although some rules just require from pos to tell illegality (for example trying to move a black piece on whites turn, it could tell you its illegal before you specify its destination)

**NOTE:** I dont know if i like board owning all of the moves and legality stuff. i think im gonna change that yes veri good

**lib** I believe the lib kind of gives you access to everything. you can do whatever you want. but i believe the basic methods are supposed to be the ones in lib.rs. **start_pos** for generating a board with pieces in starting postions, **play** a method to take a from pos and to pos to make a move, **legal_moves** to generate all legal moves, maybe for debugging or something idk. **perft_layers** to easily compare the chess api to other confirmed working apis, and see if a certain position generates the right amount of moves to a certain depth.

**Whats left** Oooof, probably a lot. but if we are just looking at functionality i believe its only pawn promotions left. If my tests work correctly and the api is actually doing what it is supposed to be doing, that said. Then write more tests, testing units and other stuff that might go wrong is always a good thing. ive been lazy so havn't done that but i think it should be fine for now, just dont do anything stupid:D

# How to use it??

## Make a board in start position and play a move
```
use hermanha_chess::{Board, MoveOk, MoveError};

fn main() -> Result<MoveOk, MoveError>{
    let mut board = Board::start_pos();
    let from = (1,1);
    let to = (2,1);

    board.play(from, to)
}   
```

## get all legal moves in specific position
```
use hermanha_chess::{Board};

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let mut board = Board::new();
    board.setup_fen(fen); 

    println!("{:?}", board.legal_moves())
}
```

## Perft analysis for certain position and move depth
```
use hermanha_chess::{Board};

fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let mut board = Board::new();
    board.setup_fen(fen);

    let depth= 3;

    println!("{:?}", board.perft_layers(depth))
}
```

