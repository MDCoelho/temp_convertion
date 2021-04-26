use std::io;

// C=(F-32)x0,5556 --> Formula to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: i32) {
    let celsius = f - 32;
    let celsius = celsius as f64 * 0.5556;
    println!("The converstion to Celsius is: {} C", celsius);
}

// F=(Cx1.8)+32 --> Formula to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: i32) {
    let fahrenheit = c as f64 * 1.8;
    let fahrenheit = fahrenheit as f64 + 32.0;
    println!("The converstion to Fahrenheit is: {} F", fahrenheit);
}


fn main() {
    loop{
        println!("###########################################");
        println!("Welcome to the temperature convertion tool!");
        println!("==> Select your option:");
        println!("1: Convert Fahrenheit to Celcius");
        println!("2: Convert Celsius to Fahrenheit");
        
        //Vai ler o user input relativo à opção escolhida
        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read line");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //Vai ler outro user input relativo à temperatura
        println!("==> Enter a temperature value:");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature).expect("Failed to read line");
        //println!("Option selected: {}",option);
        let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //Dependendo da opção escolhida vai executar funções de conversão diferentes
        if option == 1 {
            fahrenheit_to_celsius(temperature);
        } 
        else if option == 2 {
            celsius_to_fahrenheit(temperature);
        } else {
            println!("Option not available!");
            break
        }

        println!("Thank you for using this tool!");
        break
    }





}

