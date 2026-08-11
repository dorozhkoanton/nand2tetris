// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

(LOOP)
    @KBD
    D = M
    @BLACK
    D;JNE
    @color
    M = 0
    @FILL
    0;JMP

(BLACK)
    @color
    M = -1

(FILL)
    @SCREEN
    D = A
    @addr
    M = D

(FILL_LOOP)
    @addr
    D = M
    @KBD
    D = D - A
    @LOOP
    D;JGE

    @color
    D = M
    @addr
    A = M
    M = D

    @addr
    M = M + 1
    @FILL_LOOP
    0;JMP
