// push constant 17
@17
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 13
@13
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop local 1
@1
D=A
@LCL
A=M
A=A+D
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


// pop local 2
@2
D=A
@LCL
A=M
A=A+D
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


// push local 1
@1
D=A
@LCL
A=M
A=A+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// push local 2
@2
D=A
@LCL
A=M
A=A+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// push constant 7
@7
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop temp 1
@6
D=M
@x
M=D
@SP
M=M-1
A=M
D=M
@x
A=M
M=D

// push temp 1
@6
D=M
@SP
A=M
M=D
@SP
M=M+1

