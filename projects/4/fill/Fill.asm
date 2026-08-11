// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.


// state: current screen color — 0 (white) or -1 (black)
@state
M = 0

// ptr: current write position in screen memory
@SCREEN
D = A
@ptr
M = D

(LOOP)
    // Read keyboard register — nonzero means a key is held down
    @KBD
    D = M
    @WANT_BLACK
    D;JNE
    D = 0               // no key pressed → desired color = white (0)
    @CHECK
    0;JMP
(WANT_BLACK)
    D = -1              // key pressed → desired color = black (-1)

(CHECK)
    // desired - state:
    //   == 0 → color unchanged, continue filling
    //   != 0 → color changed, reset ptr and flip state
    @state
    D = D - M
    @CHANGED
    D;JNE

    // Same state: write one word to screen at current ptr position
    @state
    D = M
    @ptr
    A = M
    M = D               // screen[ptr] = state

    // Advance ptr; if ptr >= KBD then we've passed the last screen word
    @ptr
    M = M + 1
    D = M
    @KBD
    D = D - A           // ptr - 24576; negative means still inside screen
    @RESET_PTR
    D;JGE               // ptr >= KBD → wrap
    @LOOP
    0;JMP

(CHANGED)
    // Flip state: 0 → -1 (black), -1 → 0 (white)
    @state
    M = !M              // bitwise NOT, fall through to RESET_PTR

(RESET_PTR)
    // Reset ptr to beginning of screen memory and restart the loop
    @SCREEN
    D = A
    @ptr
    M = D
    @LOOP
    0;JMP
