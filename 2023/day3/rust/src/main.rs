use std::io::stdin;

fn main() {
    let mut input: Vec<String> = vec![];
    read_from_file(&mut input);
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
    let mut cords: Vec<(usize,usize)>  = vec![];

    // Find all Poses of symbols
    for (pos_s,s) in input.iter().enumerate(){
        let str: &str = s.trim();
        for (pos_c,c) in str.chars().enumerate(){
            if !c.is_digit(10) && c != '.'{
                cords.push((pos_s.clone(),pos_c));
            }
        }
    }
    let mut sum: u32 = 0;
    for (pos_s,s) in input.iter().enumerate(){
        let str: &str = s.trim();
        let mut value: u32 = 0;
        let mut start: usize = usize::MAX;
        for (pos_c,c) in str.chars().enumerate(){
            if c.is_digit(10){
                if str.len() > pos_c + 1{
                    if start == usize::MAX{
                        start = pos_c;
                    }
                    value = 10*value +  c.to_digit(10).unwrap();
                    if !str.chars().nth(pos_c+1).unwrap().is_digit(10){
                        if is_in_coord(pos_s, start, pos_c, cords.clone()){
                            sum += value;
                        }
                        //Reset
                        start = usize::MAX;
                        value = 0;
                    }
                }
                else{
                    if is_in_coord(pos_s, start, pos_c, cords.clone()){
                        value = 10*value +  c.to_digit(10).unwrap();
                        sum += value;
                    }
                }
            }
        }
    }
    println!("Sum = {sum}");
}

fn is_in_coord(line: usize, start: usize, end: usize, cords: Vec<(usize,usize)>) -> bool{
    let begin: usize;
    if start > 0{
        begin = start - 1;
    }else{
        begin = start;
    }
    for cord in cords{
        if cord.0 == line && cord.1 >= begin  && cord.1 <= end + 1{
            return true;
        }
        if cord.0 == line + 1 && cord.1 >= begin && cord.1  <= end + 1{
            return true;
        }
        if line > 0{
            if cord.0 == line - 1 && cord.1 >= begin && cord.1 <= end + 1{
                return true;
            }
        }
    }
    return false;
}

fn task2(input: Vec<String>){
    let mut cords: Vec<(usize,usize,usize,u32)>  = vec![];

    // Find all Poses of symbols
    for (pos_s,s) in input.iter().enumerate(){
        let str: &str = s.trim();
        let mut value: u32 = 0;
        let mut start: usize = usize::MAX;
        for (pos_c,c) in str.chars().enumerate(){
            if c.is_digit(10){
                if str.len() > pos_c + 1{
                    if start == usize::MAX{
                        start = pos_c;
                    }
                    value = 10*value +  c.to_digit(10).unwrap();
                    if !str.chars().nth(pos_c+1).unwrap().is_digit(10){
                        cords.push((pos_s, start, pos_c, value));
                        //Reset
                        start = usize::MAX;
                        value = 0;
                    }
                }
                else{
                    value = 10*value +  c.to_digit(10).unwrap();
                    cords.push((pos_s, start, pos_c, value));
                }
            }
        }
    }
    let mut sum: u32 = 0;
    for (pos_s,s) in input.iter().enumerate(){
        let str: &str = s.trim();
        for (pos_c,c) in str.chars().enumerate(){
            if c == '*'{
                let mut neigh: Vec<u32> = vec![];
                for (_,cord) in cords.iter().enumerate(){
                    let begin: usize = match cord.1 > 0{
                        true => cord.1 - 1,
                        false => cord.1
                    };
                    if pos_s == cord.0 && pos_c >= begin && pos_c <= cord.2 + 1{
                        neigh.push(cord.3);
                    } 
                    if pos_s + 1 == cord.0 && pos_c >= begin && pos_c <= cord.2 + 1{
                        neigh.push(cord.3);
                    } 
                    if pos_s > 0{
                        if pos_s - 1 == cord.0 && pos_c >= begin && pos_c <= cord.2 + 1{
                            neigh.push(cord.3);
                        }  
                    }
                }
                if neigh.len() == 2{
                    sum += neigh[0] * neigh[1];
                }
            }
        }
    }
    println!("Sum = {sum}");
}
