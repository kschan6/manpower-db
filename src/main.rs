use std::io::{self, Write};
use std::collections::HashMap;

enum Entry {
    Department,
    Name
}

fn main() {
    println!("Hello, manpower-db!");
    let mut db = HashMap::new();

    loop {
	let dep = prompt(Entry::Department);
	let name = prompt(Entry::Name);
	
	println!("Adding {} to {} department...", name, dep);
	add_manp(&mut db, dep, name);

	//    println!("Listing the dabase...");
	//    list_manp(db);
    }
}

fn prompt(entry: Entry) -> String {
    match entry {
	Entry::Department => print!("Enter a Department > "),
	Entry::Name => print!("Enter a name > ")
    }

    match io::stdout().flush() {
	Ok(_val) => (),
	Err(_e) => {
	    println!("Unable to flush stdout! exit abnormally");
	    std::process::exit(-1);
	}
    }

    // get user input
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
	Ok(_val) => (),
	Err(e) => println!("Error reading input: {}", e)
    }

    // remove the trailing newline
    let it = input.chars();
    if it.last() == Some('\n') {
	input.pop();
    }
    
    return input;
}

fn add_manp(db: &mut HashMap<String, Vec<String>>, dep: String, name: String) {
    let empty = Vec::new(); // Vec<String>, does not need to be mutable...
    let val = db.entry(dep).or_insert(empty);
    (*val).push(name);
    println!("vector: {:?}", val);
}
