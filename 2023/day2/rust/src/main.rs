use std::io::stdin;

fn main() {
    let mut input: Vec<String> = vec![];
    read_from_file(&mut input);
    /*
    for s in input{
        print!("{s}");
    }
    println!("");
    */
    task1(input.clone());
    task2(input.clone());
}

fn read_from_file(input: &mut Vec<String>){
    loop{
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Error");
        if s == "".to_string(){
            break;
        }
        input.push(s);
    }
}

fn task1(input: Vec<String>){
    let mut sum: i32 = 0;
    for s in input{
        let red: i32= 12;
        let green: i32 = 13;
        let blue: i32 = 14;
        let game: &str = s.as_str().split_terminator(":").collect::<Vec<&str>>()[0];
        let pulls_str: &str = s.as_str().split_terminator(":").collect::<Vec<&str>>()[1];
        let pulls: Vec<&str> = pulls_str.split_terminator(";").collect::<Vec<&str>>();
        let mut fits: bool = true;
        for str in pulls{
            let values: Vec<&str> = str.split_terminator(" ").collect::<Vec<&str>>();
            for (pos,val) in values.iter().enumerate(){
                if val.to_string() != "".to_string(){
                    let num: Result<i32,std::num::ParseIntError>  = val.parse::<i32>();
                    if num.is_ok(){
                        if values.get(pos+1).unwrap().replace(",", "").to_string().contains("red") && num.clone().unwrap() > red{
                            fits = false;
                            break;
                        }
                        if values.get(pos+1).unwrap().replace(",", "").to_string().contains("green") && num.clone().unwrap() > green{
                            fits = false;
                            break;
                        }
                        if values.get(pos+1).unwrap().replace(",", "").to_string().contains("blue") && num.clone().unwrap() > blue{
                            fits = false;
                            break;
                        }
                    }
                }
            }
        }
        if fits{
            let val: i32 = game.split_terminator(" ").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
            sum += val;
        }
    }
    println!("{sum}");
}
fn task2(input: Vec<String>){
    let mut sum: i32 = 0;
    for s in input{
        let mut red: i32 = 0;
        let mut green: i32 = 0;
        let mut blue: i32 = 0;
        let pulls_str: &str = s.as_str().split_terminator(":").collect::<Vec<&str>>()[1];
        let pulls: Vec<&str> = pulls_str.split_terminator(";").collect::<Vec<&str>>();
        for str in pulls{
            let values: Vec<&str> = str.split_terminator(" ").collect::<Vec<&str>>();
            for (pos,val) in values.iter().enumerate(){
                if val.to_string() != "".to_string(){
                    let num: Result<i32,std::num::ParseIntError>  = val.parse::<i32>();
                    if num.is_ok(){
                        if values.get(pos+1).unwrap().replace(",", "").to_string().contains("red") && num.clone().unwrap() > red{
                            red = num.clone().unwrap();
                        }
                        if values.get(pos+1).unwrap().replace(",", "").to_string().contains("green") && num.clone().unwrap() > green{
                            green = num.clone().unwrap();
                        }
                        if values.get(pos+1).unwrap().replace(",", "").to_string().contains("blue") && num.clone().unwrap() > blue{
                            blue = num.clone().unwrap();
                        }
                    }
                }
            }
        }
        sum += red * green * blue;
    }
    println!("{sum}");
}
