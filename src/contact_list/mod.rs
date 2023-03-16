use std::collections::{BTreeMap, HashMap};
use std::fs;
use itertools::Itertools;
use json::parse;

pub trait ContactListService {
    fn save_to_file(&mut self, path: &String);
    fn read_from_file(path : &String) -> ContactList;
    fn get_contacts(&mut self, offset : i32) -> Vec<&ContactDetails>;
    fn delete(&mut self, name : &String);
    fn edit(&mut self, contact_details : &ContactDetails);
    fn insert(&mut self, contact_details : &ContactDetails);
    fn find(&mut self, query: &String) -> Vec<ContactDetails>;
    fn get_all(&mut self) -> Vec<ContactDetails>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactDetails {
    pub name : String,
    pub email : String,
    pub phone : String
}

impl ContactDetails {
    pub fn print(&self){
        println!("{}: {}, {}", self.name, self.phone, self.email)
    }
}

#[derive(Debug)]
struct TrieNode {
    child : HashMap<char, TrieNode>,
    contact_details: Option<ContactDetails>,
}

#[derive(Debug)]
pub struct ContactList {
    contacts : BTreeMap<String, ContactDetails>,
    root : TrieNode,
}

impl ContactList {
    pub fn new() -> ContactList {
        ContactList {
            contacts : BTreeMap::new(),
            root :  TrieNode {
                child: HashMap::new(),
                contact_details: None,
            }
        }
    }
}

impl ContactListService for ContactList {
    fn save_to_file(&mut self, path: &String) {
        let mut data = json::JsonValue::new_array();

        self.get_all().iter().for_each(|c| {
            let mut entry = json::JsonValue::new_object();
            entry["name"] = c.name.clone().into();
            entry["phone"] = c.phone.clone().into();
            entry["email"] = c.email.clone().into();
            data.push(entry).expect("Failed to add a new entry to resulting dataset");
        });

        //println!("{}", json::stringify_pretty(&data, 4));
        let output = json::stringify_pretty(data, 4);
        fs::write(path, output).expect("Unable to write file");
    }

    fn read_from_file(path : &String) -> ContactList {
        let mut cl = ContactList::new();
        parse_input(&path).iter().for_each(|c| cl.insert(&c));

        cl
    }

    fn get_contacts(&mut self, offset : i32) -> Vec<&ContactDetails> {
        let mut result = Vec::new();

        let keys = self.contacts.keys().collect::<Vec<&String>>();

        for i in offset .. offset + 3 {
            match keys.get(i as usize) {
                Some(key) => result.push(self.contacts.get(*key).unwrap()),
                _ => {}
            }
        }

        result
    }

    fn delete(&mut self, name : &String){
        self.contacts.remove(name);

        let mut cur = &mut self.root;

        for (i,c) in name.chars().enumerate() {
            match cur.child.get_mut(&c) {
                Some(child) => {
                    cur = child;
                    if i == name.len() - 1 && cur.contact_details.is_some() {
                        cur.contact_details = None;
                        return;
                    }
                }
                None => {
                    println!("There is no such contact: {}", name);
                    return;
                }
            }
        }
    }

    fn edit(&mut self, contact_details : &ContactDetails) {
        self.insert(contact_details);
    }

    fn insert(&mut self, contact_details : &ContactDetails) {
        self.contacts.insert(contact_details.name.clone(), contact_details.clone());

        let contact = &contact_details.name;
        let len = contact.len();
        let mut cur = &mut self.root;

        for (i,c) in contact.chars().enumerate() {
            cur = cur.child.entry(c).or_insert_with(|| TrieNode {
                child: HashMap::new(),
                contact_details : None
            });

            if i == len - 1{
                cur.contact_details = Option::from(contact_details.clone());
            }
        }
    }

    fn find(&mut self, query: &String) -> Vec<ContactDetails>{
        let mut cur = &mut self.root;
        let mut container = Vec::new();

        //Getting to the end of the prefix first:
        for c in query.chars() {
            match cur.child.get_mut(&c) {
                Some(child) => {
                    cur = child;
                }
                None => {
                    return container;
                }
            }
        }

        get_all_contacts(cur, &mut container);
        container
    }

    fn get_all(&mut self) -> Vec<ContactDetails> {
        let mut result = Vec::new();
        get_all_contacts(&mut self.root, &mut result);
        result
    }


}

fn get_all_contacts(start : &mut TrieNode, container : &mut Vec<ContactDetails>){
    match &start.contact_details {
        Some(cd) => container.push(cd.clone()),
        _ => {}
    }

    start.child.iter_mut()
        .sorted_by_key(|x| x.0)
        .for_each(|x| get_all_contacts(x.1, container));
}

fn parse_input(path: &String) -> Vec<ContactDetails>{
    let file_contents =
        fs::read_to_string(path).expect("Unable to read file");
    let parsed = parse(&file_contents).unwrap();

    let mut result = Vec::new();

    for i in 0 .. parsed.len() {
        result.push(ContactDetails {
            name : parsed[i]["name"].to_string(),
            phone : parsed[i]["phone"].to_string(),
            email : parsed[i]["email"].to_string(),
        })
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::contact_list::{ContactDetails, ContactList, ContactListService};

    struct TestContext {
        contact_list: ContactList,
    }

    fn setup() -> TestContext {
        let mut contact_list = ContactList::new();

        contact_list.insert(&ContactDetails {
            name: "Tester".to_string(),
            email: "test@company.com".to_string(),
            phone: "333-444-2222".to_string(),
        });

        contact_list.insert(&ContactDetails {
            name: "Dev".to_string(),
            email: "dev@company.com".to_string(),
            phone: "333-444-2222".to_string(),
        });


        TestContext {
            contact_list
        }
    }

    #[test]
    fn test_search() {
        let mut ctx = setup();

        let contacts = ctx.contact_list.get_all();

        let search_result = ctx.contact_list.find(&String::from("T"));

        assert_eq!(2, contacts.len(),
                   "Initial dataset must contain only 2 contacts");
        assert_eq!(1, search_result.len(),
                   "Search result must contains only 1 match");
        assert_eq!("Tester", search_result[0].name);
        assert_eq!("test@company.com", search_result[0].email);
        assert_eq!("333-444-2222", search_result[0].phone);
    }

    #[test]
    fn test_insert() {
        let mut ctx = setup();

        let name = "Illia".to_string();
        let phone = "1-222-333-4444".to_string();
        let email = "user@company.com".to_string();

        ctx.contact_list.insert(&ContactDetails {
            name: name.clone(),
            email: email.clone(),
            phone: phone.clone(),
        });
        let search_result = ctx.contact_list.find(&String::from("Illi"));

        assert_eq!(3, ctx.contact_list.get_all().len(),
                   "Num of contacts after insert must be 3");
        assert_eq!(1, search_result.len(),
                   "Search result must contains only 1 match");
        assert_eq!(name, search_result[0].name);
        assert_eq!(phone, search_result[0].phone);
        assert_eq!(email, search_result[0].email);
    }

    #[test]
    fn test_edit() {
        let mut ctx = setup();

        let name = "Tester".to_string();
        let email = "tester@another.company".to_string();
        let phone = "+375 222 3333".to_string();

        ctx.contact_list.edit(&ContactDetails {
            name: name.to_string(),
            email: email.clone(),
            phone: phone.clone(),
        });

        let search_result = ctx.contact_list.find(&String::from("Tester"));

        assert_eq!(1, search_result.len(),
                   "Search result must contains only 1 match");
        assert_eq!(name, search_result[0].name);
        assert_eq!(phone, search_result[0].phone);
        assert_eq!(email, search_result[0].email);
    }

    #[test]
    fn test_delete() {
        let mut ctx = setup();

        let name = "Tester".to_string();

        ctx.contact_list.delete(&name);

        let contacts = ctx.contact_list.get_all();

        assert_eq!(1, contacts.len(),
                   "Search result must contains only 1 match");
        assert!(ctx.contact_list.find(&name).is_empty(),
                "Searching a deleted contact must return an empty dataset");
    }

    #[test]
    fn test_get_contacts_pagination() {
        let mut ctx = setup();

        let result = ctx.contact_list.get_contacts(0);

        assert_eq!(2, result.len(),
                   "If contact list is less than page size, all contacts must be returned");

        let result = ctx.contact_list.get_contacts(1);

        assert_eq!(1, result.len(),
                   "If contact list is less than page size and offset is in the middle, \
                    partial dataset must be returned");

        let result = ctx.contact_list.get_contacts(2);
        assert!(result.is_empty(),
                "If outset is beyond the contact list, empty dataset must be returned");


        let new_contacts = vec![
            ContactDetails {
                name: "x".to_string(),
                phone: "1111-1111-111".to_string(),
                email: "x@company.com".to_string()
            },
            ContactDetails {
                name: "y".to_string(),
                phone: "1111-1111-111".to_string(),
                email: "y@company.com".to_string()
            },
            ContactDetails {
                name: "z".to_string(),
                phone: "1111-1111-111".to_string(),
                email: "z@company.com".to_string()
            },
        ];

        new_contacts.iter().for_each(|c| ctx.contact_list.insert(&c));

        let result = ctx.contact_list.get_contacts(2);

        assert_eq!(3, result.len(),
                   "If there are enough elements in the dataset beyond the offset, \
                    the size of the paginated dataset must be equal to PAGE_SIZE");


        for i in 0..2 {
            assert_eq!(&new_contacts[i], result[i], "Contacts must be sorted alphabetically");
        }
    }
}