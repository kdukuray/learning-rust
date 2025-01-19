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
fn parse_string_to_int(string: &str) -> i32{
    string.trim().parse::<i32>().unwrap()
}

// This function takes in an instruction,
//does the necessary lexical analysis and executes it
fn execute_instruction(registers: &mut Vec<i32>, instruction: &str){
    let tokenized_line: Vec<Token> = tokenize_line(instruction);
    match tokenized_line[0]{
        Token::Add => {
            let mut operand_1_index: usize = Default::default();
            let mut operand_2_index: usize = Default::default();

            match tokenized_line[1]{
                Token::R(index) => operand_1_index = index,
                _ => println!("Error!")
            }
            match tokenized_line[2]{
                Token::R(index) => operand_2_index = index,
                _ => println!("Error!")
            }

            registers[operand_1_index] = registers[operand_1_index] + registers[operand_2_index];

        }
        Token::Sub => {

        }
        Token::Mul => {

        }
        Token::Div => {

        }
        Token::Set => {
            let mut operand_1_index: usize = Default::default();
            let mut value: i32 = Default::default();

            match tokenized_line[1]{
                Token::R(index) => operand_1_index = index,
                _ => println!("Error!")
            }

            match tokenized_line[2]{
                Token::Val(value_) => value = value_,
                _ => println!("Error!")
            }

            registers[operand_1_index] = value;

        }
        // How do I execute if statements?
        Token::If => {

        }
        Token::Print => {

        }
        // How do I implement jump statements?
        Token::Jump => {

        }
        _ => println!("Error!")
    }
    println!("Tokenized Line {:?}", tokenized_line);
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

fn print_registers(registers: &Vec<i32>){

}

fn main(){

    // Getting a reference to the file
    let file_path: String = String::from("C:\\Users\\kduku\\RustroverProjects\\learning_rust\\src\\asmmm.txt");
    let file_reference: File = File::open(file_path).unwrap();

    let file_reader_buffer: BufReader<File> = BufReader::new(file_reference);

    // Register logic
    let mut registers: Vec<i32> = Vec::new();

    // loop through every line, and execute the code
    for instruction in file_reader_buffer.lines(){
        execute_instruction(&mut registers, &instruction.unwrap());
    }

}