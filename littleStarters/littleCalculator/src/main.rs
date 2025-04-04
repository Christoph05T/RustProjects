use std::io;

fn get_number() -> i32{
    println!("Please enter a i32 number");
    loop {
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read line");
        if let Ok(number) = number.trim().parse::<i32>() {
            break number;
        }
        println!("Please enter a valid i32 number!:")
    }

}

fn get_operation() -> Option<String>{
    println!("Please enter an operation out of +, -, *, /   or 'q' to quit or 'AC' to delete all of your inputs you made (press ? for more explanation):");
    loop {
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read line");
        match operation.trim() {
            "+" => break Some("+".to_string()),
            "-" => break Some("-".to_string()),
            "*" => break Some("*".to_string()),
            "/" => break Some("/".to_string()),
            "q" => break None,
            "AC" => break Some("AC".to_string()),
            "?" => println!(r#"You can choose between 4 diffrent math operations:
            +: Addition
            -: Subtraction
            *: Multiplication
            /: Division
            if you want to quit the calculator just type
            'q' to quit"#),
            _ => {}

        }
        println!("Please enter a **valid** operation out of +, -, *, /, =   or q to quit (press ? for more explanation):")
    }

}



fn main() {
    println!(r#"Welcome to the calculator game!
    You can entrer a number -> opertion -> number and the calculator spits out the intermediate result!
    After that you can go on with your calculations.
    If you want to quit after an intermediate result just type 'q'"#);
    println!("=====================================================");


    let end_result = 'main: loop {

        let mut intermediate_result = get_number();

        'calculation: loop {

            let operation = get_operation().unwrap_or("None".to_string());

            match operation.as_str() {
                "None" => break 'main intermediate_result,
                "AC" => continue 'main,
                _ => {},
            }

            let number = get_number();

            match operation.as_str() {
                "+" => {
                    intermediate_result = intermediate_result + number;

                },
                "-" => {
                    intermediate_result = intermediate_result - number;

                },
                "*" => {
                    intermediate_result = intermediate_result * number;

                },
                "/" => if number == 0 {
                    println!("You can't divide by 0!");
                    continue 'calculation;
                } else {
                    intermediate_result = intermediate_result / number;

                },

                _ => break 'calculation,
            }

            println!("The intermediate result is: {}", intermediate_result);
        }



    };

    println!("The end result is: {}", end_result);
    println!("=====================================================");
    println!("Thank you for using the calculator game!");




}
