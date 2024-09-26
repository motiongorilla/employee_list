use std::collections::HashMap;
use std::io;

fn main() {
    let mut company_list: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut text_interface = String::new();
        io::stdin()
            .read_line(&mut text_interface)
            .expect("Type in text");

        if text_interface.to_lowercase().contains("end") {
            break;
        }
        if text_interface.is_empty() {
            continue;
        }

        let dpts: Vec<&str> = match text_interface.split(" to ").collect::<Vec<&str>>().get(1) {
            Some(val) => val.trim().split(",").collect(),
            None => text_interface
                .split(" from ")
                .collect::<Vec<&str>>()
                .get(1)
                .unwrap_or(&" ")
                .trim()
                .split(",")
                .collect(),
        };

        if text_interface.split_once(" ").unwrap().0 == "Add" {
            for department in &dpts {
                let dept_list = company_list
                    .entry(department.trim().to_string())
                    .or_insert(vec![]);

                let people: Vec<&str> =
                    match text_interface.split(" to ").collect::<Vec<&str>>().get(0) {
                        Some(val) => val.split_once(" ").unwrap().1.trim().split(",").collect(),
                        None => continue,
                    };

                if people.is_empty() {
                    println!("Couldn't parse people!");
                    continue;
                }

                for person in &people {
                    dept_list.push(person.trim().to_string());
                }
            }
        } else if text_interface.split_once(" ").unwrap().0 == "Remove" {
            let people: Vec<&str> =
                match text_interface.split(" from ").collect::<Vec<&str>>().get(0) {
                    Some(val) => val.split_once(" ").unwrap().1.trim().split(",").collect(),
                    None => continue,
                };

            if people.is_empty() {
                println!("Couldn't parse people!");
                continue;
            }

            for department in &dpts {
                let dept_list = company_list
                    .entry(department.trim().to_string())
                    .or_insert(vec![]);

                for person in &people {
                    let mut i: Option<usize> = None;
                    for (e, name) in dept_list.iter().enumerate() {
                        if name == person {
                            i = Some(e);
                            break;
                        }
                    }
                    if let Some(index) = i {
                        dept_list.remove(index);
                    }
                }
            }
        }
        println!("All the departments: {:?}", company_list);
    }
}

fn print_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}
