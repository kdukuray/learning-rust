use std::fs::File;
use std::io::{BufRead, BufReader};

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
fn parse_string_to_int(string: &str) -> i32{
    string.trim().parse::<i32>().unwrap()
}

// This function takes in a register token and
// return the index associated with the register
fn get_register_index(token: &Token) -> usize{
    let mut register_index: usize = Default::default();
    match token{
        Token::R(index) => register_index = *index,
        _ => println!("Error")
    }
    register_index
}

// This function takes in a value token and
// return the actual numeric value associated with the value
fn get_value_value(token: &Token) -> i32{
    let mut value:i32 = 0;
    match token{
        Token::Val(value_) => value = *value_,
        _ => println!("Error")
    }
    value
}
// This function takes in an instruction,
//does the necessary lexical analysis and executes it
fn execute_instruction(registers: &mut Vec<i32>, instruction: &str) -> i32{
    let tokenized_line: Vec<Token> = tokenize_line(instruction);
    let mut exit_code: i32 = 0;
    // if exit code is 0, do nothing ie, go to the next line of code
    // if exit_code is a positive number, jump to that line
    // if exit code is -1, an if statement condition failed, skip the next line
    match tokenized_line[0]{
        Token::Add => {
            let operand_1_index: usize = get_register_index(&tokenized_line[1]);
            let operand_2_index: usize = get_register_index(&tokenized_line[2]);
            registers[operand_1_index] += registers[operand_2_index];

        }
        Token::Sub => {
            let operand_1_index: usize = get_register_index(&tokenized_line[1]);
            let operand_2_index: usize = get_register_index(&tokenized_line[2]);
            registers[operand_1_index] -= registers[operand_2_index]

        }
        Token::Mul => {
            let operand_1_index: usize = get_register_index(&tokenized_line[1]);
            let operand_2_index: usize = get_register_index(&tokenized_line[2]);
            registers[operand_1_index] *= registers[operand_2_index]

        }
        Token::Div => {
            let operand_1_index: usize = get_register_index(&tokenized_line[1]);
            let operand_2_index: usize = get_register_index(&tokenized_line[2]);
            registers[operand_1_index] /= registers[operand_2_index]

        }
        Token::Set => {
            let operand_1_index: usize = get_register_index(&tokenized_line[1]);
            let value: i32 = get_value_value(&tokenized_line[2]);
            registers[operand_1_index] = value;

        }
        Token::If =>{
            let operand_1_index: usize = get_register_index(&tokenized_line[1]);
            let value: i32 = get_value_value(&tokenized_line[2]);
            if registers[operand_1_index] != value {
                exit_code = -1;
            }
        }
        Token::Print => {
            let operand_1_index: usize = get_register_index(&tokenized_line[1]);
            println!("{}", registers[operand_1_index]);
        }
        Token::Jump => {
            let line_number_to_jump_to: i32 = get_value_value(&tokenized_line[1]);
            exit_code = line_number_to_jump_to
        }
        _ => println!("Error!")
    }
    println!("Tokenized Line {:?}", tokenized_line);
    exit_code
}


// This function takes in a string
// and returns a tokenized representation of the string based on assembly--
fn tokenize(fragment: &str) -> Token{
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
        let register_number: usize = fragment[1..].parse::<usize>().unwrap();
        token = Token::R(register_number);
    }
        else if fragment.to_lowercase().starts_with("v"){
            let value: i32 = fragment[1..].parse::<i32>().unwrap();
            token = Token::Val(value);
        }
    else{
        token = Token::None;
    }
    token

}

// This function takes in an instruction (line of code)
// tokenizes each fragment of the instruction and returns a
// vector of tokens representing the instruction
fn tokenize_line(instruction:&str) -> Vec<Token>{
    let mut tokenized_instruction: Vec<Token> = Default::default();
    let instruction_as_vec: Vec<&str> = instruction.split(" ").collect::<Vec<&str>>();
    for fragment in instruction_as_vec.iter(){
        tokenized_instruction.push(tokenize(fragment));
    }
    tokenized_instruction
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
        exit_code = execute_instruction(&mut registers, &code_base[line_index as usize]);
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