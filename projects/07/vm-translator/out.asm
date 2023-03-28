@256
D=A
@SP
M=D

@300
D=A
@LCL
M=D

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
@TMP
M=D
@SP
M=M-1
A=M
D=M
@TMP
A=M
M=D

// pop local 2
@2
D=A
@LCL
A=M
A=A+D
D=A
@TMP
M=D
@SP
M=M-1
A=M
D=M
@TMP
A=M
M=D

// push local 1
@1
D=A
@LCL
A = M
A = A + D
D = M
@SP
A=M
M=D
@SP
M = M + 1

// push local 2
@2
D=A
@LCL
A = M
A = A + D
D = M
@SP
A=M
M=D
@SP
M = M + 1

