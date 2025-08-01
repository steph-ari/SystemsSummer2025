use std::io;

fn temperature_converter(){
    const FREEZINGPOINT: f64 = 32.0;

    fn fahrenheit_to_celsius(f: f64) -> f64 {
        (f - FREEZINGPOINT) * 5.0/9.0
    }

    fn celsius_to_fahrenheit(c: f64) -> f64 {
        (c * 9.0/5.0) + FREEZINGPOINT
    }

    let ftemp: f64 = 82.0;
    let ctemp = fahrenheit_to_celsius(ftemp);
    println!("{}°F is equal to {:.2}°C", ftemp, ctemp);

    println!("\nThe next 5 Fahrenheit integers converted into Celsius are: ");
    for i in 1..=5{
        let nextf = ftemp + i as f64;
        let nextc = fahrenheit_to_celsius(nextf);
        println!("{}°F is equal to {:.2}°C", nextf, nextc);
    }
}

fn number_analyzer(){
    let nums = [1, 2, 3, 4, 5, 6, 15, 8, 9, 10];

    fn is_even(n: i32) -> bool{
        if n%2 == 0{
            true
        }else{
            false
        }
    }

    for num in nums {
        if is_even(num) == true{
            println!("The number {} is even. ", num);

            if num%3 == 0{
                if num%5 == 0{
                    println!("FizzBuzz");
                }else{
                    println!("Fizz");
                }
            } else if num%5 == 0{
                if num%3 == 0{
                    println!("FizzBuzz");
                }else{
                    println!("Buzz");
                }
            }
        }else{
            println!("The number {} is odd. ", num);

            if num%3 == 0{
                if num%5 == 0{
                    println!("FizzBuzz");
                }else{
                    println!("Fizz");
                }
            }else if num%5 == 0{
                if num%3 == 0{
                    println!("FizzBuzz");
                }else{
                    println!("Buzz");
                }
            }
        }
    }
    let mut total = 0;
    let mut count = 0;
    while count < 10{
        total += nums[count];
        count+=1;
    }
    println!("The sum of all the numbres in the array is {}", total);

    let mut largest = nums[0];
    for num in nums{
        if num > largest{
            largest = num;
        } 
    }
    println!("The largest number in the array is {}", largest);
}

fn guessing_game(){
    let mut secret = 33;

    fn check_guess(guess: i32, secret: i32) -> i32{
        if guess == secret{
            0
        }else if guess > secret{
            1
        }else{
            -1
        }
    }

    let mut times = 0;
    while 0 < 1{
        println!("Guess the Number!: ");
        let mut guessString = String::new();

        io::stdin()
            .read_line(&mut guessString)
            .expect("Failed to read line");

        let guess: i32 = guessString
            .trim()
            .parse()
            .expect("Invalid input: Please enter a valid number.");

        if check_guess(guess, secret) == 1{
            println!("Your guess was too high!");
            times+=1;
        }else if check_guess(guess, secret) == -1{
            println!("Your guess was too low!");
            times+=1;
        } else{
            println!("Your guess was correct!");
            times+=1;
            break;
        }        
    }
    println!("It took you {} guess(s)!", times);
}

fn main() {
    //temperature_converter();
    //number_analyzer();
    guessing_game();
}