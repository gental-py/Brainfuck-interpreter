use std::{char, fs, env, io::Write, process::exit};

const ARR_SIZE: usize = 30000 as usize;


fn get_input() -> u8 {
    return loop {
        print!("\n(input) ~ ");
        std::io::stdout().flush().unwrap();
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.len() == 0 {
            println!("No input provided.");
            continue;
        }

        let c: char = input.chars().nth(0).unwrap_or(char::from_u32(0).unwrap());
        let char_value = c as usize;
        if !(0..255).contains(&char_value) {
            println!("Invalid ASCII character.");
            continue;
        }

        break char_value as u8
    };
}


# [derive(Debug)]
enum Operator {
    Inc, Dec, Left, Right, Output, Input, DevLog, LoopInit, LoopEnd,  // Standard operators
    EOF, Unknown  // Special operators
}

fn get_operator(chr: char) -> Operator {
    match chr {
        '+' => return Operator::Inc,
        '-' => return Operator::Dec,
        '<' => return Operator::Left,
        '>' => return Operator::Right,
        '.' => return Operator::Output,
        ',' => return Operator::Input,
        '@' => return Operator::DevLog,
        '[' => return Operator::LoopInit,
        ']' => return Operator::LoopEnd,
        _ => return Operator::Unknown
    }
}


# [derive(Debug)]
struct Command {
    op: Operator,
    amount: u16
}

struct PointedArray {
    size: u16,
    arr: Vec<u8>,
    pointer: u16 
} 

impl PointedArray {
    fn new() -> PointedArray {
        PointedArray {
            size: ARR_SIZE as u16,
            arr: vec![0 as u8; ARR_SIZE],
            pointer: 0
        }
    }

    fn mv_left(&mut self, amount: u16) {
        if amount > self.pointer {
            self.pointer = self.size - (amount - self.pointer);
        } else {
            self.pointer -= amount;
        }
    }

    fn mv_right(&mut self, amount: u16) {
        if (self.pointer + amount) > (self.size - 1) {
            self.pointer = amount - (self.size - self.pointer);
        } else {
            self.pointer += amount;
        }
    }

    fn increment(&mut self, amount: u16) {
        self.arr[self.pointer as usize] = self.arr[self.pointer as usize].wrapping_add(amount as u8);
    }
    
    fn decrement(&mut self, amount: u16) {
        self.arr[self.pointer as usize] = self.arr[self.pointer as usize].wrapping_sub(amount as u8);
    }

    fn get_char(&mut self) -> char {
        return char::from_u32(self.arr[self.pointer as usize] as u32).unwrap_or('?')
    }

    fn set_value(&mut self, value: u8) {
        self.arr[self.pointer as usize] = value;
    }

    fn is_zero(&mut self) -> bool {
        return self.arr[self.pointer as usize] == 0;
    }
}


fn tokenize(content: String) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();
    let mut loop_init_indexes: Vec<usize> = Vec::new();
    let mut curr_count: u16 = 0;
    let mut curr_char: char = '?';

    for c in content.chars() {
        let operator: Operator = get_operator(c);
        if matches!(operator, Operator::Unknown) { continue; };

        if matches!(operator, Operator::LoopInit) {
            if curr_char != '?' {
                let cmd: Command = Command{op: get_operator(curr_char), amount: curr_count};
                commands.push(cmd);
                curr_char = '?';
                curr_count = 0;
            }
            loop_init_indexes.push(commands.len());
            commands.push(Command{op: operator, amount: 0});
            continue;
        }

        if matches!(operator, Operator::LoopEnd) {
            if curr_char != '?' {
                let cmd: Command = Command{op: get_operator(curr_char), amount: curr_count};
                commands.push(cmd);
                curr_char = '?';
                curr_count = 0;
            }
            commands.push(Command{op: operator, amount: loop_init_indexes[loop_init_indexes.len() - 1] as u16});
            let close_index: usize = commands.len() - 1;
            if loop_init_indexes.len() == 0 {
                eprintln!("Parse error: LoopEnd before LoopInit");
                exit(1);
            }
            let matching_init: &mut Command = &mut commands[loop_init_indexes[loop_init_indexes.len() - 1]];
            matching_init.amount = close_index as u16;
            loop_init_indexes.pop();
            continue;
        }

        if curr_char == '?' {
            curr_char = c;
            curr_count = 1;
            continue;
        }

        if c != curr_char && curr_char != '?' {
            let cmd: Command = Command{op: get_operator(curr_char), amount: curr_count};
            commands.push(cmd);
            curr_char = c;
            curr_count = 1;
            continue;
        }

        if c == curr_char {
            curr_count += 1;
        }
    }
    
    if curr_char != '?' {
        let cmd: Command = Command{op: get_operator(curr_char), amount: curr_count};
        commands.push(cmd);
    }

    commands.push(Command{op: Operator::EOF, amount: 1});
    return commands;
}


fn execute(tokens: Vec<Command>) {
    let mut arr: PointedArray = PointedArray::new();
    let mut curr_index: usize = 0;

    loop {
        let cmd = &tokens[curr_index];

        match cmd.op {
            Operator::Inc => {arr.increment(cmd.amount);}
            Operator::Dec => {arr.decrement(cmd.amount);}
            Operator::Left => {arr.mv_left(cmd.amount);}
            Operator::Right => {arr.mv_right(cmd.amount);}
            Operator::Output => {
                for _ in 0..cmd.amount {
                    print!("{}", arr.get_char());
                }
            }
            Operator::Input => {arr.set_value(get_input())}
            Operator::DevLog => {
                print!("\n<@devlog pointer: {} ({})  array: ", arr.pointer, arr.arr[arr.pointer as usize]);
                for index in 0..10 {
                    if index == arr.pointer {
                        print!("[{}] ", arr.arr[index as usize]);
                    } else {
                        print!("{} ", arr.arr[index as usize]);
                    }
                }
                print!(">\n");
            }
            Operator::LoopInit => {
                if arr.is_zero() {
                    curr_index = (cmd.amount + 1) as usize;
                    continue;
                }
            }
            Operator::LoopEnd => {
                curr_index = cmd.amount as usize;
                continue;
            }

            Operator::EOF => { break; }
            Operator::Unknown => {}
        }

        curr_index += 1;
    }
}


fn main() {   
    let args: Vec<_> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("No bf file specified!");
        exit(1);
    }
    
    let file_path: &String = &args[1];
    let content: String = fs::read_to_string(file_path).expect("Cannot read input file.");
    let tokens: Vec<Command> = tokenize(content);

    execute(tokens);
}
