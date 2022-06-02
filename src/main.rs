use scanln::scanln;

fn main() {
    let max_number = scanln!("Enter destiny number: ");
    let max_number_int = max_number.parse::<i32>().unwrap();
    // for some reasons, if 5, 1..max_number_int is 4 1-4 xD
    let mut list: Vec<i32> = (2..max_number_int+1).collect();
    let mut current_index = 0;
    loop {
        if current_index >= list.len() {
            break;
        }

        let current = list[current_index];
        println!("{}", current);
        let mut multiple = current*current;
        for i in 0..list.len() {
            if i >= list.len() {
                break;
            }
            if list[i] == multiple {
                list.remove(i);
                multiple += current;
            }
            
            println!("Multiple {}", multiple);
        }
        
        current_index += 1;
    }
    
    println!("{:?}", list);
}
