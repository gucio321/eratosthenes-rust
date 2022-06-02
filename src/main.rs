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

        // search for multiplies
        let mut multiple = list[current_index];
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
    
    println!("{:?}", list);
}
