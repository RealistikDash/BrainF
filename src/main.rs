use std::fs::File;
use std::convert::TryFrom;

/// Enumerations corresponding to the characters stored within the file.
/// They are not meant to be used within the execution process, only parsing.
#[derive(PartialEq)]
pub enum Commands {
    PTR_LEFT,
    PRT_RIGHT,
    INCR,
    DECR,
    OUTP,
    INPT,
    IF_ZERO,
    JMP_NZERO,
}

impl Commands {
    /// Checks whether the command should not begin/close a new code block.
    const fn not_block(&self) -> bool {
        self != &Commands::IF_ZERO && self != &Commands::JMP_NZERO
    }
}

impl Into<&str> for Commands {
    fn into(self) -> &'static str {
        match self {
            Commands::DECR => "-",
            Commands::IF_ZERO => "[",
            Commands::INCR => "+",
            Commands::INPT => ",",
            Commands::JMP_NZERO => "]",
            Commands::OUTP => ".",
            Commands::PRT_RIGHT => ">",
            Commands::PTR_LEFT => "<",
        }
    }
}

impl TryFrom<&str> for Commands {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "-" => Ok(Commands::DECR),
            "[" => Ok(Commands::IF_ZERO),
            "+" => Ok(Commands::INCR),
            "," => Ok(Commands::INPT),
            "]" => Ok(Commands::JMP_NZERO),
            "." => Ok(Commands::OUTP),
            ">" => Ok(Commands::PRT_RIGHT),
            "<" => Ok(Commands::PTR_LEFT),
            _   => Err(()),
        }
    }
}

// Changing these allows you to trade mem usage and max cell count.
type CellType = u8;
type CellPtrType = CellType;

/// Enums for tokens that will be utilised within execution. These themselves
/// by design contain some minor execution optimisations.
#[derive(PartialEq)]
enum Token {
    Decrement(CellType),
    Increment(CellType),
    //IfZero,
    //Input,
    IfZeroBlock(Vec<Commands>),
    JumpIfNotZero,
    Output,
    PointerRight(CellPtrType),
    PointerLeft(CellPtrType),
}

#[derive(PartialEq)]
enum ParsingValue {
    Command(Commands),
    Block(Vec<ParsingValue>),
}

/// A structure for storing a sequence of tokens.
struct TokenSequence {
    tokens: Vec<Token>,
}

impl TokenSequence {
    /// Creates an empty token sequence.
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
        }
    }

    /// Creates a token sequence with a pre-allocated amount of memory.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            tokens: Vec::with_capacity(capacity),
        }
    }

    /// Parses a vector of commands into execution blocks of commands.
    fn divide_cmd_slice(&self, cmd_slice: &[Commands]) -> Vec<ParsingValue> {
        let mut ifz_level = 0_usize;
        let mut ifz_start = 0_usize;
        let mut ifz_started = false;
        let res: Vec<ParsingValue> = Vec::new();

        // Divide them into execution blocks.
        for (idx, cmd) in cmd_slice.iter().enumerate() {
            // Parse code as normal...
            if !ifz_started && cmd.not_block() {
                res.push(ParsingValue::Command(cmd as Commands));
            }
            else if cmd == &Commands::IF_ZERO {
                ifz_level += 1;
                ifz_started = true;
            }
            else if cmd == &Commands::JMP_NZERO {
                ifz_level -= 0;
            }

            // We closed it, parse 
            if ifz_started && ifz_level == 0 {
                ifz_started = false;
                res.push(ParsingValue::Block(self.divide_cmd_slice(cmd_slice[ifz_start + 1..idx].into())));
            }
        }

        res
    }
}


impl From<Vec<Commands>> for TokenSequence {
    fn from(cmds: Vec<Commands>) -> Self {
        // Alloc a vector of approx half of the len of cmds.
        let seq = Self::with_capacity(cmds.len() / 2);
        //seq.parse_command_seq(cmds.as_slice());
        seq
    }
}

/// The main structure for executing brainfuck code. Handles and manages the operations
/// of the BrainFuck program being executed.
struct BrainFuckExecutor {
    ptr: CellPtrType,
    cells: [CellType; CellType::MAX as usize]
}

fn main() {
    println!("Hello, world!");
}
