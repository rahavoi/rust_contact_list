use crate::contact_list_app::contact_list_app::{ContactDetails, ContactList};
use std::{env};

mod contact_list_app;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Please provide a path to JSON file with initial set of contacts as 1st arg!");
        eprintln!("Please provide a path to output JSON file as 2nd arg!");
        return;
    }

    let input_file = &args[1];
    let output_file = &args[2];
    let mut cl = ContactList::read_from_file(input_file);

    run_demo(&mut cl);
    cl.save_to_file(output_file);

    println!("\n (Updated contact list has been saved to {})", output_file)
}

//TODO: make this interactive - it'll be more fun.
fn run_demo(cl : &mut ContactList){
    let contacts = cl.get_all();

    println!("Initial dataset contains {} contacts: ", contacts.len());
    contacts.iter().for_each(|c| c.print());

    let query = "Ali".to_string();
    println!("\nLets find all users starting with : {}", query);
    cl.find(&query).iter().for_each(|c| c.print());

    let new_user_name = "Tester".to_string();
    println!("\nLets add a new user {} into our contacts list..", new_user_name);
    cl.insert(ContactDetails {
        name : new_user_name,
        email : "test@company.com".to_string(),
        phone : "333-444-2222".to_string(),
    });

    let edit_user_name = "Alice".to_string();
    println!("Add edit user {}..", edit_user_name);
    cl.edit(ContactDetails {
        name : edit_user_name,
        email : "alice@anothercompany.com".to_string(),
        phone : "000-000-0001".to_string()
    });


    let user_name_to_delete = "Bob".to_string();
    println!("And delete user {}..", user_name_to_delete);
    cl.delete(&user_name_to_delete);

    println!("\nUpdated contact list: ");
    cl.get_all().iter().for_each(|c| c.print());

    println!("\nLet's fetch some contacts by setting offset to 1: ");
    cl.get_contacts(1).iter().for_each(|c| c.print());

    println!("\nThat's it folks!");
}