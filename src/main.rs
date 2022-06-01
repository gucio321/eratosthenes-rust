use scanln::scanln;

fn main() {
    let max_number = scanln!("Enter destiny number: ");
    let max_number_int = max_number.parse::<i32>().unwrap();
    // for some reasons, if 5, 1..max_number_int is 4 1-4 xD
    let mut list: Vec<i32> = (1..max_number_int+1).collect();
    for i in list {
        println!("{}", i);
    }
}
