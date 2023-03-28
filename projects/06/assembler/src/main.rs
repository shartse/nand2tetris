use std::{fs::File, io::{self, BufRead, Write}, env, collections::HashMap};


struct SymbolTable(HashMap<String, usize>);

impl SymbolTable {
    fn new() -> Self {
        let mut symbols = HashMap::new();
        symbols.insert("R0".to_string(), 0);
        symbols.insert("R1".to_string(), 1);
        symbols.insert("R2".to_string(), 2);
        symbols.insert("R3".to_string(), 3);
        symbols.insert("R4".to_string(), 4);
        symbols.insert("R5".to_string(), 5);
        symbols.insert("R6".to_string(), 6);
        symbols.insert("R7".to_string(), 7);
        symbols.insert("R8".to_string(), 8);
        symbols.insert("R9".to_string(), 9);
        symbols.insert("R10".to_string(), 10);
        symbols.insert("R11".to_string(), 11);
        symbols.insert("R12".to_string(), 12);
        symbols.insert("R13".to_string(), 13);
        symbols.insert("R14".to_string(), 14);
        symbols.insert("R15".to_string(), 15);
        
        symbols.insert("SCREEN".to_string(), 16384);
        symbols.insert("KBD".to_string(), 24576);
        symbols.insert("SP".to_string(), 0);
        symbols.insert("LCL".to_string(), 1);
        symbols.insert("ARG".to_string(), 2);
        symbols.insert("THIS".to_string(), 3);
        symbols.insert("THAT".to_string(), 4);
        SymbolTable(symbols)
    }

    fn insert(&mut self, k: String, val: usize) -> Option<usize> {
        self.0.insert(k, val)
    }

    fn get(&self, k : &str) -> Option<&usize> {
        self.0.get(k)
    }
    
}


#[derive(Debug, PartialEq, Clone)]
enum Program {
    Label(String),
    Instr(Instr)
}

#[derive(Debug, PartialEq, Clone)]
struct Comp(String);

#[derive(Debug, PartialEq, Clone)]
enum Instr {
    A(Value),
    C(Dest, Comp, Jump),
}

#[derive(Debug, PartialEq, Clone)]
struct Dest {
    a: bool,
    d: bool,
    m: bool, 
}

impl Dest {
    fn new() -> Self {
        Self { a: false, m: false, d: false }
    }
    
    fn from_string(input: &str) -> Dest {
        Dest{a: input.contains("A"), d: input.contains("D"), m: input.contains("M")}
    }

    fn to_binary(&self) -> String {
       format!("{}{}{}", self.a as usize, self.d as usize, self.m as usize) 
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Jump {
    lt: bool,
    eq: bool,
    gt: bool,
}

impl Jump {
    fn new() -> Self {
        Self { lt: false, eq: false, gt: false }
    }

    fn from_string(input: &str) -> Self {
        let (lt, eq, gt) =   match input {
            "JGT" => (false, false, true),
            "JEQ" => (false, true, false),
            "JGE" => (false, true, true),
            "JLT" => (true, false, false),
            "JNE" => (true, false, true),
            "JLE" => (true, true, false),
            "JMP" => (true, true, true),
            _ => panic!("Invalid jump string {:?}", input)
        };
        Jump{lt, eq, gt}
    }

    fn to_binary(&self) -> String {
       format!("{}{}{}", self.lt as usize, self.eq as usize, self.gt as usize) 
    }
}


#[derive(Debug, PartialEq, Clone)]
enum Value {
    Literal(usize),
    Variable(String),
}

impl Value {
    fn from_string(input: &str) -> Self{
        if let Ok(n) = input.parse::<usize>() {
            Value::Literal(n)
        } else {
            Value::Variable(input.to_string())
        }
    }

    fn to_binary(&self) -> String {
        match self {
            Value::Literal(l) => {
                let mut bin_num = format!("{:b}", l);
                let prefix_len = 16 - bin_num.len();
                for _i in 0..prefix_len {
                    bin_num.insert(0, '0')
                }
                bin_num
            },
            Value::Variable(_var) => panic!("Should not have any variables at this point: {:?}", self),
        }
    }
}

impl Program {
    fn from_string(input : &str) -> Self {
        match input.chars().next() {
            Some(c) => {
                match c {
                    '(' => Program::Label(input[1..input.len()-1].to_string()),
                    _ => Program::Instr(Instr::from_string(input)),
                }
            },
            None => panic!("Expect non-empty strings to parse"),
        }
    }
}

impl Instr {
    fn from_string(input : &str) -> Self {
        match input.chars().next() {
            Some(c) => {
                match c {
                    '@' => Instr::A(Value::from_string(&input[1..])), 
                    _ => {
                        // dest=comp;jump
                        let mut comp = input.to_string();
                        let mut dest = Dest::new();
                        let mut jump = Jump::new();
                        if let Some((dest_str, rest))  = input.split_once("=") {
                            dest = Dest::from_string(dest_str);
                            comp = rest.to_string();
                        }
                        if let Some((front, jump_str))  = comp.split_once(";") {
                            jump = Jump::from_string(jump_str);
                            comp = front.to_string();
                        }
                        Instr::C(dest, Comp(comp), jump) 
                    },
                }
            },
            None => panic!("Expect non-empty strings to parse"),
        }
    }

    fn to_binary(&self) -> String {
        match self {
            Instr::A(val) => val.to_binary(),
            Instr::C(dest, comp, jump) => {
                // 1 1 1 a c1 c2 c3 c4 c5 c6 d1 d2 d3 j1 j2 j3
                let bin = "111".to_string();
                let comp_bin = match comp.0.as_str() {
                    "0" =>  "0101010",
                    "1" =>  "0111111",
                    "-1" => "0111010",
                    "D" =>  "0001100",
                    "A" =>  "0110000", 
                    "!D" => "0001101",
                    "!A" => "0110001",
                    "-D" => "0001111",
                    "-A" => "0110011", 
                    "D+1" =>"0011111",
                    "A+1" =>"0110111",
                    "D-1" =>"0001110",
                    "A-1" =>"0110010",
                    "D+A" =>"0000010",
                    "D-A" =>"0010011",
                    "A-D" =>"0000111",
                    "D&A" =>"0000000",
                    "D|A" =>"0010101",
                     "M" => "1110000",
                    "!M" => "1110001",
                    "-M" => "1110011",
                    "M+1" =>"1110111",
                    "M-1" =>"1110010",
                    "D+M" =>"1000010",
                    "D-M" =>"1010011",
                    "M-D" =>"1000111",
                    "D&M" =>"1000000",
                    "D|M" =>"1010101",
                    _ => panic!("Invalid computation: {:?}", comp),
                };
                bin + comp_bin + &dest.to_binary() + &jump.to_binary()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Instr, Value, Dest, Jump, Comp};

    #[test]
    fn parse_a_instr() {
        let code = "@6";
        assert_eq!(Instr::A(Value::Literal(6)), Instr::from_string(code));
        assert_eq!(Instr::A(Value::Literal(6)).to_binary(),   "0000000000000110");
        assert_eq!(Instr::A(Value::Literal(56)).to_binary(),  "0000000000111000");
        assert_eq!(Instr::A(Value::Literal(1001)).to_binary(),"0000001111101001");
    }

     #[test]
    fn parse_c_instr() {
        let code = "D-1";
        let instr = Instr::C(Dest::new(), Comp("D-1".to_string()), Jump::new());
        assert_eq!(instr, Instr::from_string(code));
        assert_eq!(instr.to_binary(),                            "1110001110000000");
        assert_eq!(Instr::from_string("D|M").to_binary(),        "1111010101000000");
        assert_eq!(Instr::from_string("D|A").to_binary(),        "1110010101000000");
        assert_eq!(Instr::from_string("MD=M+1").to_binary(),     "1111110111011000");
        assert_eq!(Instr::from_string("MD=M+1;JGE").to_binary(), "1111110111011011");
        assert_eq!(Instr::from_string("M=A").to_binary(), "1110110000001000");
    }
}

fn main() {
    // Accept a file name
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let out_path = &args[2];
    let file = File::open(path).unwrap();
    let mut program = Vec::new();
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
        program.push(Program::from_string(trimmed_line))
    }
    
    // First pass: Go through commands, seperate labels and instructions, building symbol table
    let mut symbols = SymbolTable::new();

    let mut instructions = Vec::new();
    let mut idx = 0;
    for instr in program {
        match instr {
            Program::Label(label) => {
                println!("Inserting label: {:?}. {:?}", label, idx);
                symbols.insert(label, idx);
            },
            Program::Instr(instr) => {
                instructions.push(instr);
                idx += 1;
            },
        };
    }

    // Second pass: go through instructions and replace variables with value of label. Variables that don't
    // have labels are allocated in registers starting at 16, and replaced with their value
    let mut next_var = 16;
    let literals : Vec<Instr> = instructions.into_iter().map(|i| match i {
        Instr::A(val) => match val {
            Value::Literal(l) => Instr::A(Value::Literal(l)),
            Value::Variable(v) => {
                match symbols.get(v.as_str()) {
                    Some(idx) => Instr::A(Value::Literal(*idx)),
                    None => {
                        symbols.insert(v, next_var);
                        let instr = Instr::A(Value::Literal(next_var));
                        next_var+=1;
                        instr
                    },
                }
            },
        }
        Instr::C(_, _, _) => i,
    }).collect();
    println!("Symbols: {:?}", symbols.0); 
    let mut out_file = File::create(out_path).unwrap();
    for instr in literals {
        out_file.write_all(instr.to_binary().as_bytes()).unwrap();
        out_file.write_all(b"\n").unwrap();
    }
}