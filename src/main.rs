

enum Operation{
    Addition,
    Subtraction,
    Multiplication,
    Division,
    None
}

fn print_registers(registers: &[i32]){
    println!("Registers: {:?}", registers);
}

fn print_menu(){
    println!("Here are your operations: ");
    println!("1. To set the value of a register");
    println!("2. To perform operations on two registers");
}

fn set_value(registers: &mut [i32]){
    println!("complete later");
}

fn binary_operation(registers: &mut [i32]){
    println!("complete later");
}
fn main(){
    let mut register: [i32; 8] = Default::default();
    let mut user_command: String = String::new();
    let input_handler: std::io::Stdin = std::io::stdin();
    print_registers(&register);
    print_menu();
    input_handler.read_line(&mut user_command).expect("Failed to read user command");
    user_command = user_command.trim().to_string();
    if user_command == "1"{
        set_value(&mut register);
    }
    else if user_command == "2"{
        binary_operation(&mut register);
    }
    else{
        println!("Complete later");
    }


}