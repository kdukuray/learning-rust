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
    R(i32),
    #[default]
    None

}

// This function takes in an instruction,
//does the necessary lexical analysis and executes it
fn execute_instruction(instruction: &str){
    let tokenized_line: Vec<Token> = tokenize_line(instruction);
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
        let register_number: i32 = fragment[1..].parse::<i32>().unwrap();
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

fn main(){

    // Getting a reference to the file
    let file_path: String = String::from("C:\\Users\\kduku\\RustroverProjects\\learning_rust\\src\\asmmm.txt");
    let file_reference: File = File::open(file_path).unwrap();

    let file_reader_buffer: BufReader<File> = BufReader::new(file_reference);

    // loop through every line, and execute the code
    for instruction in file_reader_buffer.lines(){
        execute_instruction(&instruction.unwrap());
    }

}