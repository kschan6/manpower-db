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
	println!();

	// println!("Listing the dabase...");
	// list_manp(&db);

	if prompt_end() {
	    break;
	}
    }

    println!("Listing names of all departments...");
    let dep_sorted = sort_dep(&db);
    list_manp_all(&mut db, &dep_sorted);

    println!("Listing names of a single department...");
    let dep = prompt_dep();
    list_manp_dep(&db, &dep);
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

// check for user's willingness to add the next entry
fn prompt_end() -> bool {
    print!("Next entry (Y or N)? > ");

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
	Ok(_) => (),
	Err(e) => println!("Error reading input: {}", e)
    }

    // an input starting with a y (case-insensitive) means user does NOT want to end
    let mut it = input.chars();
    if let Some('Y') | Some('y') = it.next() {
	false
    } else {
	true
    }
}

// prompt for a department name
fn prompt_dep() -> String {
    print!("What department? > ");

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
	Ok(_) => (),
	Err(e) => println!("Error reading input: {}", e)
    }

    // remove the trailing newline
    let it = input.chars();
    if it.last() == Some('\n') {
	input.pop();
    }
    
    return input;
}

// add a name to a department
fn add_manp(db: &mut HashMap<String, Vec<String>>, dep: String, name: String) {
    let empty = Vec::new(); // Vec<String>, does not need to be mutable...
    let val = db.entry(dep).or_insert(empty);
    (*val).push(name);
//    println!("vector: {:?}", val);
}

// list ALL names in ALL departments
fn _list_manp(db: &HashMap<String, Vec<String>>) {
    for (dep, list) in db {
	println!("{} Department:", dep);

	let it = list.iter().enumerate();

	for (idx, val) in it {
	    let s = format!("{}. {}", idx + 1, val); // idx starts from 0, so make it starts from 1
	    println!("{}", s);
	}
    }
}

// sort department names in the database
fn sort_dep(db: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut v = Vec::new();
    
    for k in db.keys() {
	v.push(k.clone()); // we create new strings rather than borrowing from the hash map
    }

    v.sort();
    v
}

// list names of all departments (the deparment names are sorted)
fn list_manp_all(db: &mut HashMap<String, Vec<String>>, deps: &Vec<String>) {
    for dep in deps {
	println!("{} Department:", dep);

	// need to get a mutable refence to call sort()
	if let Some(names) = db.get_mut(dep) {
	    names.sort();
	    
	    for (idx, name) in names.iter().enumerate() {
		println!("{}. {}", idx + 1, name);
	    }
	}
	println!();
   }
}

// list names of a single department
fn list_manp_dep(db: &HashMap<String, Vec<String>>, dep: &String) {
    match db.get(dep) {
	Some(names) => {
	    for (idx, name) in names.iter().enumerate() {
		println!("{}. {}", idx + 1, name);
	    }
	},

	None => println!("No entry in {} department", dep)
    }
}
