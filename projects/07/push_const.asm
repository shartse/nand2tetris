@256 // A = 256
D=A  // D = 256
@SP  // A = SP=(0)
M=D  // RAM[0] = 256

@10
D=A
@LCL
M=D

// push constant i
//@17  // A = 17
//D=A  // D = 17
//@SP  // A = SP (0)
//A=M  // A = RAM[0] = 256
//M=D  // RAM[256] = 17
//@SP  // A = 0
//M=M+1 // RAM[0] ++


// find @LCL + i
// Push local i
@1 // A = 1
D=A // D = 1
@LCL // A = LCL
A = M // A = RAM[LCL]
A = A + D // A = RAM[LCL] + i
D = M // D = RAM[(RAM[LCL] + i)]
@SP // A = SP
A=M // A = RAM[SP] 
M=D // RAM[256] = D
@SP
M = M + 1


// pop local i
// Take the top of the stack and set RAM[RAM[LCL] + i] to it. Decrease SP

@2  // A = 2
D=A // D = 2
@LCL // A = LCL
A = M // A = RAM[LCL]
A = A + D // RAM[LCL] + i
D=A
@tmp
M=D // RAM[tmp] = lcl

@SP // A = SP
M = M - 1

A=M // A = RAM[SP] 
D=M

@tmp
A=M
M=D



