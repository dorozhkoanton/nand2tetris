// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/4/Fill.asm

// Runs an infinite loop that listens to the keyboard input. 
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel. When no key is pressed, 
// the screen should be cleared.

// state: 0 = белый, -1 = чёрный
@state
M = 0

@SCREEN
D = A
@ptr
M = D

(LOOP)
    @KBD
    D = M
    @WANT_BLACK
    D;JNE
    D = 0
    @CHECK
    0;JMP
(WANT_BLACK)
    D = -1

(CHECK)
    @state
    D = D - M       // desired - state
    @CHANGED
    D;JNE           // ≠ 0 → смена состояния

    // Состояние не изменилось: пишем одно слово
    @state
    D = M
    @ptr
    A = M
    M = D           // screen[ptr] = state

    @ptr
    M = M + 1
    D = M
    @KBD
    D = D - A
    @RESET_PTR
    D;JGE           // ptr >= KBD → wrap
    @LOOP
    0;JMP

(CHANGED)
    @state
    M = !M          // 0 ↔ -1, fall-through в RESET_PTR

(RESET_PTR)
    @SCREEN
    D = A
    @ptr
    M = D
    @LOOP
    0;JMP
