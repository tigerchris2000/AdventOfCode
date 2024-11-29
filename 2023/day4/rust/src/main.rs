use std::io::stdin;
fn main() {
    let mut input: Vec<String> = vec![];
    read_from_file(&mut input);
    // task1(input.clone());
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
    let mut sum: u32 = 0;
    for mut s in input{
        s = s.replace("\n", "");
        let tmp: &str = s.split_terminator(":").collect::<Vec<&str>>()[1];
        let find: &str = tmp.split_terminator("|").collect::<Vec<&str>>()[0];
        let wins: &str = tmp.split_terminator("|").collect::<Vec<&str>>()[1];
        let mut value: u32 = 0;
        // println!("{s}");
        for mut x in find.split_terminator(" ").collect::<Vec<&str>>(){
            x = x.trim();
            let num: Result<u32, std::num::ParseIntError> = x.parse::<u32>();
            if num.is_ok(){
                for mut y in wins.split_terminator(" ").collect::<Vec<&str>>(){
                    y = y.trim();
                    // println!("{x}, {y}");
                    let num2: Result<u32, std::num::ParseIntError> = y.parse::<u32>();
                    if num2.is_ok(){
                        /*
                        let val1 = num.clone().unwrap();
                        let val2 = num2.clone().unwrap();
                        println!("{val1}, {val2}");
                        */
                        if num2 == num{
                            value = match value == 0 {
                                true => 1,
                                false => value * 2,
                            }
                        }
                    }
                }
            }
        }
        //println!("{value}");
        sum += value; 
    }
    println!("{sum}");
}

fn task2(input: Vec<String>){
    let mut sum: u32 = 0;
    let mut findings: Vec<u32> = vec![];
    for mut s in input{
        s = s.replace("\n", "");
        let tmp: &str = s.split_terminator(":").collect::<Vec<&str>>()[1];
        let find: &str = tmp.split_terminator("|").collect::<Vec<&str>>()[0];
        let wins: &str = tmp.split_terminator("|").collect::<Vec<&str>>()[1];
        let mut value: u32 = 0;
        // println!("{s}");
        for mut x in find.split_terminator(" ").collect::<Vec<&str>>(){
            x = x.trim();
            let num: Result<u32, std::num::ParseIntError> = x.parse::<u32>();
            if num.is_ok(){
                for mut y in wins.split_terminator(" ").collect::<Vec<&str>>(){
                    y = y.trim();
                    // println!("{x}, {y}");
                    let num2: Result<u32, std::num::ParseIntError> = y.parse::<u32>();
                    if num2.is_ok(){
                        /*
                        let val1 = num.clone().unwrap();
                        let val2 = num2.clone().unwrap();
                        println!("{val1}, {val2}");
                        */
                        if num2 == num{
                            value += 1;
                        }
                    }
                }
            }
        }
        //println!("{}",value);
        findings.push(value);
    }
    sum = calc(findings.clone(),0);
    println!("{sum}");
}

fn calc(mut vals: Vec<u32>,d: u32) -> u32{
    let mut sum: u32 = vals.len() as u32;
    let mut values = vals.clone();
    for (i, val) in vals.iter_mut().enumerate(){
        for j in 1..=*val{
            sum += 1;
        }
    }
    return sum;
}
fn printer(vals: Vec<u32>,d: u32){
    for _ in 0..d{
        print!("  ");
    }
    for v in vals{
        print!("{v},");
    }
    println!("");
}
