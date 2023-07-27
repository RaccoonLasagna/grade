use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1{
        println!("{}", "Input something!")
    } else{
        let grade:f64 = args[1].parse().unwrap();
        if grade > 100.0 {
            println!("{}", "Invalid score");
        } else if grade >= 95.0 {
            println!("{}","Excellent with A+")
        } else if grade >= 81.0 {
            println!("{}","A")
        } else if grade >= 71.0 {
            println!("{}","B")
        } else if grade >= 61.0 {
            println!("{}","C")
        } else if grade >= 50.0 {
            println!("{}","D")
        } else if grade >= 0.0 {
            println!("{}","Failed with F")
        } else {
            println!("{}","Invalid score")
        }
    }


}