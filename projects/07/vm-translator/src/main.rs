use std::{
    env,
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
};

#[derive(Debug)]
enum VMCommand {
    Stack(StackOp),
    BinaryArithmeticLogical(BinOp),
    UnaryArithmeticLogical(UnOp),
}

#[derive(Debug)]
enum BinOp {
    Add,
    Sub,
    Eq,
    Gt,
    Lt,
    And,
    Or,
}

static INC_SP: &'static str = "@SP\nM=M+1\n";
static DEC_SP: &'static str = "@SP\nM=M-1\n";

impl VMCommand {
    fn from_string(input: &str) -> Self {
        if input.starts_with("push") || input.starts_with("pop") {
            VMCommand::Stack(StackOp::from_string(input))
        } else {
            match input {
                "add" => VMCommand::BinaryArithmeticLogical(BinOp::Add),
                "sub" => VMCommand::BinaryArithmeticLogical(BinOp::Sub),
                "eq" => VMCommand::BinaryArithmeticLogical(BinOp::Eq),
                "gt" => VMCommand::BinaryArithmeticLogical(BinOp::Gt),
                "lt" => VMCommand::BinaryArithmeticLogical(BinOp::Lt),
                "and" => VMCommand::BinaryArithmeticLogical(BinOp::And),
                "or" => VMCommand::BinaryArithmeticLogical(BinOp::Or),
                "neg" => VMCommand::UnaryArithmeticLogical(UnOp::Neg),
                "not" => VMCommand::UnaryArithmeticLogical(UnOp::Not),
                _ => panic!("Unrecognized symbol: {:?}", input),
            }
        }
    }

    fn translate(&self, filename: &str, idx: usize) -> String {
        match self {
            VMCommand::Stack(instr) => instr.translate(filename),
            VMCommand::BinaryArithmeticLogical(op) => match op {
                BinOp::Add => Self::arithmetic("M=M+D"),
                BinOp::Sub => Self::arithmetic("M=M-D"),
                BinOp::Eq => Self::comparison("JEQ", idx),
                BinOp::Gt => Self::comparison("JGT", idx),
                BinOp::Lt => Self::comparison("JLT", idx),
                BinOp::And => Self::arithmetic("M=M&D"),
                BinOp::Or => Self::arithmetic("M=M|D"),
            },
            VMCommand::UnaryArithmeticLogical(op) => {
                format!(
                    "{DEC_SP}\
                     @SP\n\
                     A=M\n\
                     {}\n\
                     {INC_SP}",
                    match op {
                        UnOp::Neg => "D=0\nM=D-M",
                        UnOp::Not => "M=!M",
                    }
                )
            }
        }
    }

    fn arithmetic(op: &str) -> String {
        format!(
            "{DEC_SP}\
            @SP\n\
            A=M\n\
            D=M\n\
            {DEC_SP}\
            @SP\n\
            A=M\n\
            {op}\n\
            {INC_SP}"
        )
    }

    fn comparison(op: &str, i: usize) -> String {
        format!(
            "{DEC_SP}\
            @SP\n\
            A=M\n\
            D=M\n\
            {DEC_SP}\
            @SP\n\
            A=M\n\
            D=M-D\n\
            @EQUAL{i}\n\
            D;{op}\n\
            @SP\n\
            A=M\n\
            M=0\n\
            @END{i}\n\
            0;JEQ\n\
            (EQUAL{i})\n\
            @SP\n\
            A=M\n\
            M=-1\n\
            (END{i})\n\
            {INC_SP}"
        )
    }
}

#[derive(Debug)]
enum UnOp {
    Neg,
    Not,
}
#[derive(Debug)]
enum StackOp {
    Push(Segment, u32),
    Pop(Segment, u32),
}

#[derive(Debug)]
enum Segment {
    Local,
    Argument,
    This,
    That,
    Constant,
    Static,
    Pointer,
    Temp,
}

impl StackOp {
    fn from_string(input: &str) -> Self {
        let command: Vec<&str> = input.split(' ').collect();
        let memmory_segment = match command
            .get(1)
            .expect("push/pop command requires a memory segment")
        {
            &"local" => Segment::Local,
            &"argument" => Segment::Argument,
            &"this" => Segment::This,
            &"that" => Segment::That,
            &"constant" => Segment::Constant,
            &"static" => Segment::Static,
            &"temp" => Segment::Temp,
            &"pointer" => Segment::Pointer,
            other => panic!("Unexpected value instead of memory segment: {:?}", other),
        };
        let val = command
            .get(2)
            .expect("push/pop command requires a value")
            .parse::<u32>()
            .expect("push/pop value must be an integer");

        match command.get(0).expect("Invalid push/pop command") {
            &"push" => StackOp::Push(memmory_segment, val),
            &"pop" => StackOp::Pop(memmory_segment, val),
            other => panic!("Unexpected value instead of push/pop: {:?}", other),
        }
    }

    fn translate(&self, filename: &str) -> String {
        match self {
            StackOp::Push(seg, i) => match seg {
                Segment::Local => StackOp::segment_push("LCL", *i),
                Segment::Argument => StackOp::segment_push("ARG", *i),
                Segment::This => StackOp::segment_push("THIS", *i),
                Segment::That => StackOp::segment_push("THAT", *i),
                Segment::Constant => StackOp::push_constant(*i),
                Segment::Static => StackOp::var_push(&(format!("{}.{}", filename, i))),
                Segment::Pointer => StackOp::var_push(if *i == 0 { "THIS" } else { "THAT" }),
                Segment::Temp => StackOp::var_push(&(5 + *i).to_string()),
            },
            StackOp::Pop(seg, i) => match seg {
                Segment::Local => StackOp::segment_pop("LCL", *i),
                Segment::Argument => StackOp::segment_pop("ARG", *i),
                Segment::This => StackOp::segment_pop("THIS", *i),
                Segment::That => StackOp::segment_pop("THAT", *i),
                Segment::Constant => panic!("There is no pop constant command"),
                Segment::Static => StackOp::var_pop(&(format!("{}.{}", filename, i))),
                Segment::Pointer => StackOp::var_pop(if *i == 0 { "THIS" } else { "THAT" }),
                Segment::Temp => StackOp::var_pop(&(5 + *i).to_string()),
            },
        }
    }

    // Push the constant n to the top of the stack
    fn push_constant(n: u32) -> String {
        format!(
            "@{n}\n\
            D=A\n\
            {}",
            StackOp::push_d()
        )
    }

    // Push the value stored in D to the top of the stack
    fn push_d() -> String {
        format!(
            "@SP\n\
            A=M\n\
            M=D\n\
            {INC_SP}"
        )
    }

    // Push the variable store in this variable to the top of the stack
    fn var_push(variable: &str) -> String {
        format!(
            "@{variable}\n\
            D=M\n\
            {}",
            StackOp::push_d()
        )
    }

    // Push the value at this index in this segment to the top of the stack
    fn segment_push(segment: &str, index: u32) -> String {
        format!(
            "@{index}\n\
            D=A\n\
            @{segment}\n\
            A=M\n\
            A=A+D\n\
            D=M\n\
            {}",
            StackOp::push_d()
        )
    }

    // Pop the top of the stack to the location tracked by D
    fn pop_to_d() -> String {
        format!(
            "@x\n\
            M=D\n\
            {DEC_SP}\
            A=M\n\
            D=M\n\
            @x\n\
            A=M\n\
            M=D\n"
        )
    }

    // Pop the top of the stack to this variable
    fn var_pop(variable: &str) -> String {
        format!(
            "@{variable}\n\
            D=A\n\
            {}",
            StackOp::pop_to_d()
        )
    }

    // Pop the top of the stack to this index in this segment
    fn segment_pop(segment: &str, index: u32) -> String {
        format!(
            "@{index}\n\
            D=A\n\
            @{segment}\n\
            A=M\n\
            A=A+D\n\
            D=A\n\
            {}\n",
            StackOp::pop_to_d()
        )
    }
}

fn main() {
    println!("Hello, world!");
    // Accept a file name
    let args: Vec<String> = env::args().collect();
    let in_path = &args
        .get(1)
        .expect("Please supply input file as the first argument");
    let out_path = &args
        .get(2)
        .expect("Please supply an output file as the second argument");
    let file = File::open(in_path).unwrap();
    let path = Path::new(in_path);
    let classname = path.file_stem().unwrap();
    let mut hack_program = Vec::new();
    let mut instr = 0;
    // Read a lines out of the file, ignoring whitespace, parse them into Instructions, put them in a Vec
    for line in io::BufReader::new(file).lines().map(|x| x.unwrap()) {
        let trimmed_line = if let Some((code, _comment)) = line.split_once("//") {
            code.trim()
        } else {
            line.trim()
        };

        if trimmed_line.len() == 0 || trimmed_line.starts_with("//") {
            continue;
        };
        hack_program.push(format!("// {}", trimmed_line));

        let vm_command = VMCommand::from_string(trimmed_line);
        println!("translating {:?}", vm_command);
        hack_program.push(vm_command.translate(classname.to_str().unwrap(), instr));
        instr += 1;
    }

    let mut out_file = File::create(out_path).unwrap();
    for instr in hack_program {
        out_file.write_all(instr.as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();
    }
}