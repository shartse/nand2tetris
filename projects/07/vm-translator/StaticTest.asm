// push constant 111
@111
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 333
@333
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 888
@888
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop static 8
@../MemoryAccess/StaticTest/StaticTest.8
D=A
@x
M=D
@SP
M=M-1
A=M
D=M
@x
A=M
M=D

// pop static 3
@../MemoryAccess/StaticTest/StaticTest.3
D=A
@x
M=D
@SP
M=M-1
A=M
D=M
@x
A=M
M=D

// pop static 1
@../MemoryAccess/StaticTest/StaticTest.1
D=A
@x
M=D
@SP
M=M-1
A=M
D=M
@x
A=M
M=D

// push static 3
@../MemoryAccess/StaticTest/StaticTest.3
D=M
@SP
A=M
M=D
@SP
M=M+1

// push static 1
@../MemoryAccess/StaticTest/StaticTest.1
D=M
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

// push static 8
@../MemoryAccess/StaticTest/StaticTest.8
D=M
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

