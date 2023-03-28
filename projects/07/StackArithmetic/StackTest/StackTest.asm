// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// eq
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@EQUAL2
D;JEQ
@SP
A=M
M=0
@END2
0;JEQ
(EQUAL2)
@SP
A=M
M=-1
(END2)
@SP
M=M+1

// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1

// eq
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@EQUAL5
D;JEQ
@SP
A=M
M=0
@END5
0;JEQ
(EQUAL5)
@SP
A=M
M=-1
(END5)
@SP
M=M+1

// push constant 16
@16
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// eq
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@EQUAL8
D;JEQ
@SP
A=M
M=0
@END8
0;JEQ
(EQUAL8)
@SP
A=M
M=-1
(END8)
@SP
M=M+1

// push constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

// lt
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@EQUAL11
D;JLT
@SP
A=M
M=0
@END11
0;JEQ
(EQUAL11)
@SP
A=M
M=-1
(END11)
@SP
M=M+1

// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 892
@892
D=A
@SP
A=M
M=D
@SP
M=M+1

// lt
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@EQUAL14
D;JLT
@SP
A=M
M=0
@END14
0;JEQ
(EQUAL14)
@SP
A=M
M=-1
(END14)
@SP
M=M+1

// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 891
@891
D=A
@SP
A=M
M=D
@SP
M=M+1

// lt
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@EQUAL17
D;JLT
@SP
A=M
M=0
@END17
0;JEQ
(EQUAL17)
@SP
A=M
M=-1
(END17)
@SP
M=M+1

// push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// gt
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@EQUAL20
D;JGT
@SP
A=M
M=0
@END20
0;JEQ
(EQUAL20)
@SP
A=M
M=-1
(END20)
@SP
M=M+1

// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 32767
@32767
D=A
@SP
A=M
M=D
@SP
M=M+1

// gt
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@EQUAL23
D;JGT
@SP
A=M
M=0
@END23
0;JEQ
(EQUAL23)
@SP
A=M
M=-1
(END23)
@SP
M=M+1

// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 32766
@32766
D=A
@SP
A=M
M=D
@SP
M=M+1

// gt
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
D=M-D
@EQUAL26
D;JGT
@SP
A=M
M=0
@END26
0;JEQ
(EQUAL26)
@SP
A=M
M=-1
(END26)
@SP
M=M+1

// push constant 57
@57
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 31
@31
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 53
@53
D=A
@SP
A=M
M=D
@SP
M=M+1

// add
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
M=M+D
@SP
M=M+1

// push constant 112
@112
D=A
@SP
A=M
M=D
@SP
M=M+1

// sub
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
M=M-D
@SP
M=M+1

// neg
@SP
M=M-1
@SP
A=M
D=0
M=D-M
@SP
M=M+1

// and
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
M=M&D
@SP
M=M+1

// push constant 82
@82
D=A
@SP
A=M
M=D
@SP
M=M+1

// or
@SP
M=M-1
@SP
A=M
D=M
@SP
M=M-1
@SP
A=M
M=M|D
@SP
M=M+1

// not
@SP
M=M-1
@SP
A=M
M=!M
@SP
M=M+1

