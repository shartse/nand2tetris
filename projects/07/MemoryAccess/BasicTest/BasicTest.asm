// push constant 10
@10
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop local 0
@0
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


// push constant 21
@21
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 22
@22
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop argument 2
@2
D=A
@ARG
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


// pop argument 1
@1
D=A
@ARG
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


// push constant 36
@36
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop this 6
@6
D=A
@THIS
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


// push constant 42
@42
D=A
@SP
A=M
M=D
@SP
M=M+1

// push constant 45
@45
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop that 5
@5
D=A
@THAT
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


// pop that 2
@2
D=A
@THAT
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


// push constant 510
@510
D=A
@SP
A=M
M=D
@SP
M=M+1

// pop temp 6
@11
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

// push local 0
@0
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

// push that 5
@5
D=A
@THAT
A=M
A=A+D
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

// push argument 1
@1
D=A
@ARG
A=M
A=A+D
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

// push this 6
@6
D=A
@THIS
A=M
A=A+D
D=M
@SP
A=M
M=D
@SP
M=M+1

// push this 6
@6
D=A
@THIS
A=M
A=A+D
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

// push temp 6
@11
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

