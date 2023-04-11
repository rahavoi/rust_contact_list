use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use crate::contact_list::{ContactList, ContactListService};
use crate::controllers::{add_contact, AppState, delete_contact, edit_contact, get_all_contacts, get_contacts};

mod contact_list;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    //#After pulling my hair for half an hour trying to understand why the state of my app is not consistent
    //(after inserting a bunch of test items, GET kept giving me different results back!)
    // I found this in the actix-web documentation:
    //State initialized inside the closure passed to HttpServer::new is local to the worker thread and may become de-synced if modified.
    //To achieve globally shared state, it must be created outside of the closure passed to HttpServer::new and moved/cloned in.

    //#mind-blowing!

    let data = web::Data::new(AppState {
        app_name: String::from("Contact List"),
        contact_list : Mutex::new(ContactList::read_from_file(&String::from("/Users/irahavoi/IdeaProjects/contact_list/samples/example.json"))),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(get_contacts)
            .service(get_all_contacts)
            .service(add_contact)
            .service(edit_contact)
            .service(web::resource("/contacts/{name}")
                         .route(web::delete().to(delete_contact)))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}