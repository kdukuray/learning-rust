// This is a register based calculator app.
// The app has 8 registers and only supports 4 operations (+, -, *, /)
use std::io::Write;

#[derive(Default)]
enum Operation{
    Addition,
    Subtraction,
    Multiplication,
    Division,
    #[default]
    None
}
// prints the values inside the registers.
// Also prints the index of each register right above it.
fn print_registers(registers: &[i32]){
    println!("\t0|\t1|\t2|\t3|\t4|\t5|\t6|\t7|");
    for register in registers {
        print!("\t{}", register);
        std::io::stdout().flush().unwrap();
    }
    println!();
}

// Prints out the user menu
fn print_menu(){
    println!("Here are your operations: ");
    println!("1. To set the value of a register");
    println!("2. To perform operations on two registers");
    println!("All other numerical input will terminate the program.");
    println!("All other non numerical inputs are invalid and will crash the program");
}

// Sets the value of specific registers.
fn set_value(registers: &mut [i32]){
    let input_handler: std::io::Stdin = std::io::stdin();
    let mut index_as_str: String = Default::default();
    let mut index: usize = 0;
    let mut value_as_str: String = String::new();
    let mut value: i32 = 0;
    println!("Enter the index of the register whose value you would like to set:");
    input_handler.read_line(&mut index_as_str).expect("Failed to register index");
    index = index_as_str.trim().parse::<usize>().expect("Failed to parse register index");
    println!("Please enter a value for the register being updated");
    input_handler.read_line(&mut value_as_str).expect("Failed to read value");
    value = value_as_str.trim().parse::<i32>().expect("Failed to parse register value");
    registers[index] = value;
}

// Performs binary operations on register values and stores result in the specified register.
fn binary_operation(registers: &mut [i32]){
    let input_handler: std::io::Stdin = std::io::stdin();
    let mut symbol: Operation = Default::default();
    let mut user_cmd: String = Default::default();
    let mut operand_1_index_as_str: String = Default::default();
    let mut operand_2_index_as_str: String = Default::default();
    let mut result_index_as_str: String = Default::default();
    let mut operand_1_index: usize = Default::default();
    let mut operand_2_index: usize = Default::default();
    let mut result_index: usize = Default::default();
    let mut result: i32 = 0;
    println!("Please enter an operation symbol");
    input_handler.read_line(&mut user_cmd).expect("Failed to read symbol");
    if user_cmd.trim() == "+"{
        symbol = Operation::Addition;
    }
    else if user_cmd.trim() == "-"{
        symbol = Operation::Subtraction;
    }
    else if user_cmd.trim() == "*"{
        symbol = Operation::Multiplication;
    }
    else if user_cmd.trim() == "/"{
        symbol = Operation::Division;
    }
    else{
        symbol = Operation::None;
    }
    println!("Please enter the index of the register for the first operand");
    input_handler.read_line(&mut operand_1_index_as_str).expect("Failed to read operand_1 index");
    operand_1_index = operand_1_index_as_str.trim().parse::<usize>().expect("Failed to parse operand_1 index");
    println!("Please enter the index of the register for the secind operand");
    input_handler.read_line(&mut operand_2_index_as_str).expect("Failed to read operand_2 index");
    operand_2_index = operand_2_index_as_str.trim().parse::<usize>().expect("Failed to parse operand_2 index");
    println!("Please enter the index of the result");
    input_handler.read_line(&mut result_index_as_str).expect("Failed to read result index");
    result_index = result_index_as_str.trim().parse::<usize>().expect("Failed to parse result index");

    match symbol{
        Operation::Addition => result = registers[operand_1_index] + registers[operand_2_index],
        Operation::Subtraction => result = registers[operand_1_index] - registers[operand_2_index],
        Operation::Multiplication => result = registers[operand_1_index] * registers[operand_2_index],
        Operation::Division => result = registers[operand_1_index] / registers[operand_2_index],
        _ => println!("Invalid operation symbol")
    }
    registers[result_index] = result;

}
fn main(){
    let mut registers: [i32; 8] = Default::default();
    let mut user_command: String = Default::default();
    let input_handler: std::io::Stdin = std::io::stdin();
    print_registers(&registers);
    print_menu();
    input_handler.read_line(&mut user_command).expect("Failed to read user command");
    user_command = user_command.trim().to_string();
    while user_command.parse::<i32>().unwrap() >= 0 && user_command.parse::<i32>().unwrap() <= 2{

        if user_command == "1"{
            set_value(&mut registers);
        }
        else if user_command == "2"{
            binary_operation(&mut registers);
        }
        print_registers(&registers);
        print_menu();

        user_command.clear();
        input_handler.read_line(&mut user_command).expect("Failed to read user command");
        user_command = user_command.trim().to_string();
    }

}