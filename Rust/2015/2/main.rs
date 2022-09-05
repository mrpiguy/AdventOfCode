use std::fs;

fn main(){
    let filename = "2.txt".to_string();
    part1(&filename);
    part2(&filename);
}

fn part1(filename: &String){
    let lines = read_lines(filename);
}