use std::io::stdin;

fn main() {
    // let input: Vec<String> = vec!["1abc2".to_string(),"pqr3stu8vwx".to_string(),"a1b2c3d4e5f".to_string(), "treb7uchet".to_string()]; 
    // let input: Vec<String> = vec!["two1nine".to_string(),"eightwothree".to_string(),"abcone2threexyz".to_string(),"xtwone3four".to_string(),
    //                                "4nineeightseven2".to_string(),"zoneight234".to_string(),"7pqrstsixteen".to_string(),]; 
    
    let mut input: Vec<String> = vec![]; 
    loop{
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Error");
        if s == "".to_string(){
            break;
        }
        input.push(s);
    }
    
    task1(input.clone());
    task2(input.clone());
}

fn task1(input: Vec<String>){
    let ints: String = "0123456789".to_string();
    let mut count: i32 = 0;
    for s in input{
        let mut first: char = ' ';
        let mut last: char = ' ';
        for c in s.chars(){
           if ints.contains(c){
                if first == ' '{
                    first = c;
                }
                last = c;
           }
        }
        let var: &str = &(first.to_string() + &last.to_string());
        let tmp: i32 = var.parse().unwrap();

        count += tmp;
    }
    println!("{}",count);
}

fn task2(input: Vec<String>){
    
    let ints: Vec<String> = vec![
        "0".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
        "8".to_string(),
        "9".to_string(),
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string()
    ];
    
    /*
    let ints: Vec<&str> = vec![
        "0",
        "1",
        "2",
        "3",
        "4",
        "5",
        "6",
        "7",
        "8",
        "9",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];
    */
    let mut count: i32 = 0;
    for s in input{
        let mut cs: Vec<char> = vec![];
        for c in s.chars(){
            cs.push(c);
        }
        let mut first: i32 = -1;
        let mut last: i32 = -1;
        for i in 0..cs.len(){
            let mut str: String = String::new(); 
            for j in 0..7{
                if j+i < cs.len(){
                    str += &cs.get(j+i).unwrap().to_string();
                    if ints.contains(&str){
                        if first == -1{
                            first = to_char(str.clone());
                        }
                        last = to_char(str.clone());
                        break;
                    }
                }
            }
        }
        count += first * 10;
        count += last;
    }
    println!("{}",count);
}
fn to_char(s: String) -> i32{
    let i: Result<i32,core::num::ParseIntError> = s.parse::<i32>();
    if i.is_ok(){
        return s.parse::<i32>().unwrap();
    }
    if s == "one".to_string(){
        return 1;
    }
    if s == "two".to_string(){
        return 2;
    }
    if s == "three".to_string(){
        return 3;
    }
    if s == "four".to_string(){
        return 4;
    }
    if s == "five".to_string(){
        return 5;
    }
    if s == "six".to_string(){
        return 6;
    }
    if s == "seven".to_string(){
        return 7;
    }
    if s == "eight".to_string(){
        return 8;
    }
    if s == "nine".to_string(){
        return 9;
    }
    return -1;
}
