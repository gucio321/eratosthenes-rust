use scanln::scanln;
use std::time::Instant;

fn main() {
    let max_number = scanln!("Enter destiny number: ");
    let max_number_int = max_number.parse::<i32>().unwrap();
    let time_before = Instant::now();
    println!("{:?}", get_prime_numbers(max_number_int));
    println!("duration of operation: {:?}", time_before.elapsed());
}

fn get_prime_numbers(max:i32) -> Vec<i32> {
    let mut list: Vec<i32> = (2..max+1).collect();
    let last_index = *list.last().expect("no last index?");
    let mut current_index = 0;
    loop {
        // search for multiplies
        let multiple = list[current_index];
        if multiple*2 > last_index {
            break;
        }
        
        let mut i = current_index + 1;
        
        loop {
            if i >= list.len() {
                break;
            }
            
            let current_idx = list.get(i).expect("list is empty");
            if *current_idx != multiple && current_idx % multiple == 0 {
                list.remove(i);
            }
            
            i += 1;
        }
        
        current_index += 1;
    }
    
    return list;
}
