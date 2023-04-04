use std::{
    env,
    fs::{self, File},
    io::{self, BufRead, Write},
    path::{Path, PathBuf},
};

#[derive(Debug)]
enum VMCommand {
    Stack(StackOp),
    BinaryArithmeticLogical(BinOp),
    UnaryArithmeticLogical(UnOp),
    Label(String),
    GoTo(String),
    IfGoTo(String),
    Call(String, usize),
    Function(String, usize),
    Return,
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
        match input {
            "return" => VMCommand::Return,
            "add" => VMCommand::BinaryArithmeticLogical(BinOp::Add),
            "sub" => VMCommand::BinaryArithmeticLogical(BinOp::Sub),
            "eq" => VMCommand::BinaryArithmeticLogical(BinOp::Eq),
            "gt" => VMCommand::BinaryArithmeticLogical(BinOp::Gt),
            "lt" => VMCommand::BinaryArithmeticLogical(BinOp::Lt),
            "and" => VMCommand::BinaryArithmeticLogical(BinOp::And),
            "or" => VMCommand::BinaryArithmeticLogical(BinOp::Or),
            "neg" => VMCommand::UnaryArithmeticLogical(UnOp::Neg),
            "not" => VMCommand::UnaryArithmeticLogical(UnOp::Not),
            _ => {
                let (command, args) = input.split_once(" ").expect("Unrecognized symbol");
                match command {
                    "push" | "pop" => VMCommand::Stack(StackOp::from_string(input)),
                    "label" => VMCommand::Label(args.to_string()),
                    "goto" => VMCommand::GoTo(args.to_string()),
                    "if-goto" => VMCommand::IfGoTo(args.to_string()),
                    "call" => {
                        let (name, nargs) = args
                            .split_once(" ")
                            .expect("call requires a name and number of arguments");

                        VMCommand::Call(name.to_string(), nargs.parse().unwrap())
                    }
                    "function" => {
                        let (name, nvars) = args
                            .split_once(" ")
                            .expect("function requires a name and number of variables");
                        VMCommand::Function(name.to_string(), nvars.parse().unwrap())
                    }
                    _ => panic!("Unrecognized symbol: {:?}", input),
                }
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
            VMCommand::Label(label) => {
                format!("({label})")
            }
            VMCommand::GoTo(label) => {
                format!(
                    "@{label}\n\
                    0;JEQ\n"
                )
            }
            VMCommand::IfGoTo(label) => {
                // pop the top of the stack into D, load adder of label, jump if D != 0
                format!(
                    "{DEC_SP}\n\
                    @SP\n\
                    A=M\n\
                    D=M\n\
                    @{label}\n\
                    D;JNE\n"
                )
            }
            VMCommand::Call(name, nargs) => Self::call(name, *nargs, idx),
            VMCommand::Function(name, nvars) => Self::function(name, *nvars),
            VMCommand::Return => Self::function_return(),
        }
    }

    fn init() -> String {
        let call_sys_init = Self::call("Sys.init", 0, 0);
        format!(
            "@256\n\
            D=A\n\
            @SP\n\
            M=D\n\
            {call_sys_init}
            "
        )
    }

    fn call(name: &str, nargs: usize, i: usize) -> String {
        // Push the location in code that we will return to - the value of a label?
        let ret_addr = format!("{name}return{i}");
        let push_d = StackOp::push_d();
        let push_lcl = StackOp::var_push("LCL");
        let push_arg = StackOp::var_push("ARG");
        let push_this = StackOp::var_push("THIS");
        let push_that = StackOp::var_push("THAT");
        format!(
            "@{ret_addr}\n\
            D=A\n\
            {push_d}\n\
            {push_lcl}\n\
            {push_arg}\n\
            {push_this}\n\
            {push_that}\n\
            @SP\n\
            D=M\n\
            @5\n\
            D=D-A\n\
            @{nargs}\n\
            D=D-A\n\
            @ARG\n\
            M=D\n\
            @SP\n\
            D=M\n\
            @LCL\n\
            M=D\n\
            @{name}\n\
            0;JEQ\n\
            ({ret_addr})\n"
        )
    }

    fn function(name: &str, nvars: usize) -> String {
        let push_0 = StackOp::push_constant(0);
        let init_local_vars = push_0.repeat(nvars);
        format!(
            "({name})\n\
            {init_local_vars}\n"
        )
    }

    /*
    endFrame = LCL // gets the address at the frame’s end
    retAddr = *(endFrame – 5) // gets the return address
    *ARG = pop() // puts the return value for the caller
    SP = ARG + 1 // repositions SP
    THAT = *(endFrame – 1) // restores THAT
    THIS = *(endFrame – 2) // restores THIS
    ARG = *(endFrame – 3) // restores ARG
    LCL = *(endFrame – 4) // restores LCL
    goto retAddr // jumps to the return address the global stack
     */
    fn function_return() -> String {
        let pop_to_d = StackOp::pop_to_d();
        let restore_segments: String = vec!["THAT", "THIS", "ARG", "LCL"]
            .iter()
            .map(|name| {
                format!(
                    "@R14\n\
                AM=M-1\n\
                D=M\n\
                @{name}\n\
                M=D\n"
                )
            })
            .collect();

        format!(
            "@LCL\n\
            D=M\n\
            @R14\n\
            M=D\n\
            @5\n\
            A=D-A\n\
            D=M\n\
            @R15\n\
            M=D\n\
            {pop_to_d}\n\
            @ARG\n\
            A=M\n\
            M=D\n\
            @ARG\n\
            D=M\n\
            @SP\n\
            M=D+1\n\
            {restore_segments}\n\
            @R15\n\
            A=M\n\
            0;JEQ\n"
        )
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
            "@R13\n\
            M=D\n\
            {DEC_SP}\
            A=M\n\
            D=M\n\
            @R13\n\
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

fn translate_file(path: &PathBuf, instr: &mut usize) -> Vec<String> {
    let file = File::open(path).unwrap();
    let classname = path.file_stem().unwrap();
    let mut hack_program = Vec::new();
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
        hack_program.push(vm_command.translate(classname.to_str().unwrap(), *instr));
        *instr += 1;
    }
    hack_program
}

fn main() {
    // Accept a file name or a directory name
    let args: Vec<String> = env::args().collect();
    let in_path = &args
        .get(1)
        .expect("Please supply input file as the first argument");
    let out_path = &args
        .get(2)
        .expect("Please supply an output file as the second argument");

    let mut instr = 0;
    let in_path = Path::new(in_path);

    let hack_program = if let Some(ext) = in_path.extension() {
        let mut hack_program = Vec::new();
        if ext == "vm" {
            hack_program.append(&mut translate_file(&in_path.to_path_buf(), &mut instr));
        }
        hack_program
    } else {
        let mut hack_program = vec![VMCommand::init()];
        for entry in fs::read_dir(in_path).unwrap() {
            let entry = entry.unwrap();
            let sub_dir_path = entry.path();
            println!("Dir contents: {:?}", sub_dir_path);
            if let Some(ext) = sub_dir_path.extension() {
                if ext == "vm" {
                    hack_program.append(&mut translate_file(&sub_dir_path, &mut instr));
                }
            }
        }
        hack_program
    };

    let mut out_file = File::create(out_path).unwrap();
    for instr in hack_program {
        out_file.write_all(instr.as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();
    }
}
