use std::fs;

fn main(){
    let filename = "1.txt".to_string();
    part1(&filename);
    part2(&filename);
}

fn part1(filename: &String){
    let contents = fs::read_to_string(filename).expect("error");
    let mut floor = 0;
    for chr in contents.chars(){
        if chr == '(' {
            floor = floor + 1;
        }else{
            floor = floor - 1;
        }
    }
    println!("{}",floor)
}

fn part2(filename: &String){
    let contents = fs::read_to_string(filename).expect("error");
    let mut floor = 0;
    let mut counter = 1;
    for chr in contents.chars(){
        if chr == '(' {
            floor = floor + 1;
        }else{
            floor = floor - 1;
        }
        if floor == -1 {
            break;
        }
        counter = counter + 1;

    }
    println!("{}",counter)

}