use scanln::scanln;

fn main() {
    let max_number = scanln!("Enter destiny number: ");
    let max_number_int = max_number.parse::<i32>().unwrap();
    println!("{:?}", get_prime_numbers(max_number_int));
}

fn get_prime_numbers(max:i32) -> Vec<i32> {
    let mut list: Vec<i32> = (2..max+1).collect();
    let mut current_index = 0;
    let mut list_len= list.len();
    loop {
        if current_index >= list_len {
            break;
        }

        // search for multiplies
        let multiple = list[current_index];
        let mut i = current_index + 1;
        loop {
            if i >= list_len {
                break;
            }
            
            let current_idx = list.get(i).expect("list is empty");
            if *current_idx != multiple && current_idx % multiple == 0 {
                list.remove(i);
                list_len -= 1;
            }
            
            i += 1;
        }
        
        current_index += 1;
    }
    
    return list;
}
