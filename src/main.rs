use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

#[derive(Default, Debug)]
enum Token{
    Add,
    Sub,
    Mul,
    Div,
    Set,
    If,
    Print,
    Jump,
    R(usize),
    Val(i32),
    #[default]
    None
}

// This functions takes in an integer as a string and
// returns the integer representation of the string
fn parse_string_to_int(string: &str) -> Result<i32, ParseIntError>{
    string.trim().parse::<i32>()
}

// This function takes in a register token and
// return the index associated with the register
fn get_register_index(token: &Token) -> Result<usize, String>{
    let mut register_index: usize = Default::default();
    let mut register_index_error: bool = false;
    match token{
        Token::R(index) => register_index = *index,
        _ => register_index_error = true
    }
    if !register_index_error && register_index >= 0 && register_index < 10{
        Ok(register_index)
    }
    else{
        Err("Register index invalid! Register indexes must numbers from 0 and 9 ".to_string())
    }
}

// This function takes in a value token and
// return the actual numeric value associated with the value
fn get_value_value(token: &Token) -> Result<i32, String>{
    let mut value:i32 = 0;
    let mut value_value_error: bool = false;
    match token{
        Token::Val(value_) => value = *value_,
        _ => value_value_error = true
    }
    if !value_value_error{
        Ok(value)
    }
    else{
        Err("Val' value is invalid! Value must be a number (32 bit integer)".to_string())
    }
}
// This function takes in an instruction,
//does the necessary lexical analysis and executes it
fn execute_instruction(registers: &mut Vec<i32>, instruction: &str, line_index: &i32) -> i32{
    let tokenized_line: Vec<Token> = tokenize_line(instruction).unwrap_or_else(|error_msg|{
        panic!("Error on line {}. {}", line_index+1, error_msg)
    });
    let mut exit_code: i32 = 0;
    // if exit code is 0, do nothing ie, go to the next line of code
    // if exit_code is a positive number, jump to that line
    // if exit code is -1, an if statement condition failed, skip the next line
    match tokenized_line[0]{
        Token::Add | Token::Sub | Token::Mul | Token::Div => {
            let operand_1_index: usize = get_register_index(&tokenized_line[1]).unwrap_or_else(|error_msg: String|{
                panic!("Error on line {}. {}", line_index+1, error_msg)
            });
            let operand_2_index: usize = get_register_index(&tokenized_line[2]).unwrap_or_else(|error_msg: String|{
                panic!("Error on line {}. {}", line_index+1, error_msg)
            });

            if let Token::Add = tokenized_line[0]{
                registers[operand_1_index] += registers[operand_2_index];
            }
            else if let Token::Sub = tokenized_line[0]{
                registers[operand_1_index] -= registers[operand_2_index];
            }
            else if let Token::Mul = tokenized_line[0]{
                registers[operand_1_index] *= registers[operand_2_index];
            }
            else if let Token::Div = tokenized_line[0]{
                registers[operand_1_index] /= registers[operand_2_index];
            }

        }
        Token::Set | Token::If => {
            let operand_1_index: usize = get_register_index(&tokenized_line[1]).unwrap_or_else(|error_msg: String|{
                panic!("Error on line {}. {}", line_index+1, error_msg)
            });
            let value: i32 = get_value_value(&tokenized_line[2]).unwrap_or_else(|error_msg: String|{
                panic!("Error on line {}. {}", line_index+1, error_msg)
            });

            if let Token::Set = tokenized_line[0]{
                registers[operand_1_index] = value;
            }
            else if let Token::If = tokenized_line[0]{
                if registers[operand_1_index] != value {
                    exit_code = -1;
                }
            }
        }

        Token::Print => {
            let operand_1_index: usize = get_register_index(&tokenized_line[1]).unwrap_or_else(|error_msg: String|{
                panic!("Error on line {}. {}", line_index+1, error_msg)
            });
            println!("{}", registers[operand_1_index]);
        }
        Token::Jump => {
            let line_number_to_jump_to: i32 = get_value_value(&tokenized_line[1]).unwrap_or_else(|error_msg: String|{
                panic!("Error on line {}. {}", line_index+1, error_msg)
            });
            exit_code = line_number_to_jump_to
        }
        _ => panic!("Error on line {}. Instructions must start with  one of the following \
        Mnemonics: ADD, SUB, MUL, DIV, SET, IF, PRINT, JUMP", line_index+1)
    }
    //This line is for debugging, delete later
    println!("Tokenized Line {:?}", tokenized_line);
    exit_code
}

// This function takes in a string
// and returns a tokenized representation of the string based on assembly--
fn tokenize(fragment: &str) -> Result<Token, ParseIntError>{
    let mut token: Token = Default::default();
    if fragment.to_lowercase() == "add"{
        token = Token::Add;
    }
    else if fragment.to_lowercase() == "sub"{
        token = Token::Sub;
    }
    else if fragment.to_lowercase() == "mul"{
        token = Token::Mul;
    }
    else if fragment.to_lowercase() == "div"{
        token = Token::Div;
    }
    else if fragment.to_lowercase() == "set"{
        token = Token::Set;
    }
    else if fragment.to_lowercase() == "if"{
        token = Token::If;
    }
    else if fragment.to_lowercase() == "print"{
        token = Token::Print;
    }
    else if fragment.to_lowercase() == "jump"{
        token = Token::Jump;
    }
    else if fragment.to_lowercase().starts_with("r"){
        let register_index_result: Result<usize, ParseIntError> = fragment[1..].parse::<usize>();
        match register_index_result{
            Ok(register_index) => token = Token::R(register_index),
            Err(err) => return Err(err)
        }
    }
    else if fragment.to_lowercase().starts_with("v"){
        let value_result: Result<i32, ParseIntError> = fragment[1..].parse::<i32>();
        match value_result{
            Ok(value) => token = Token::Val(value),
            Err(err) => return Err(err)
        }
    }
    else{
        token = Token::None;
    }
    Ok(token)

}

// This function takes in an instruction (line of code)
// tokenizes each fragment of the instruction and returns a
// vector of tokens representing the instruction
fn tokenize_line(instruction:&str) -> Result<Vec<Token>, String>{
    let mut tokenized_instruction: Vec<Token> = Default::default();
    let instruction_as_vec: Vec<&str> = instruction.split(" ").collect::<Vec<&str>>();
    for fragment in instruction_as_vec.iter(){
        let tokenize_result :Result<Token, ParseIntError> = tokenize(fragment);
        match tokenize_result{
            Ok(token) => tokenized_instruction.push(token),
            Err(err) => return Err(format!("\"{}\" is invalid. R and V Mnemonics must contain by a number.", fragment))
        }
    }
    Ok(tokenized_instruction)
}

//This function prints the registers
fn print_registers(registers: &Vec<i32>){
    println!("{:?}", registers);

}

fn main(){
    // Getting a reference to the file and loading it to a buffer
    let file_path: String = String::from("C:\\Users\\kduku\\RustroverProjects\\learning_rust\\src\\asmmm.txt");
    let file_reference: File = File::open(file_path).unwrap();
    let file_reader_buffer: BufReader<File> = BufReader::new(file_reference);

    // Register logic (We will only have 9 registers
    let mut registers: Vec<i32> = Vec::from([0; 10]);

    // loop through every line, and execute the code
    let mut line_index: i32 = 0;
    // exit_code represents the status the of the most recently executed line of code
    let mut exit_code: i32 = 0;
    let mut code_base: Vec<String> = Default::default();
    for line_of_code in file_reader_buffer.lines(){
        code_base.push(line_of_code.unwrap());
    }

    while line_index < code_base.len() as i32{
        exit_code = execute_instruction(&mut registers, &code_base[line_index as usize], &line_index);
        print_registers(&registers);

        if exit_code > 0{
            line_index = exit_code;
        }
        else if exit_code == 0{
            line_index += 1;
        }
        else{
            line_index += 2;
        }
    }
    println!("Program was executed successfully");

}