use std::sync::Mutex;
use actix_web::{get, HttpResponse, post, put, Responder, web};
use serde::Deserialize;
use crate::contact_list::{ContactDetails, ContactList, ContactListService};

pub struct AppState {
    pub app_name: String,
    pub contact_list : Mutex<ContactList>,
}

#[derive(Deserialize)]
pub struct QueryParams {
    offset: Option<i32>,
}

#[get("/contacts/all")]
pub async fn get_all_contacts(data : web::Data<AppState>) -> impl Responder {
    let clones : Vec<ContactDetails> = clone(data.contact_list.lock().unwrap().get_all());
    web::Json(clones)
}

#[get("/contacts")]
pub async fn get_contacts(data : web::Data<AppState>, params : web::Query<QueryParams>) -> impl Responder {
    let offset = params.offset.unwrap_or(0);
    let clones : Vec<ContactDetails> = clone(data.contact_list.lock().unwrap().get_contacts(offset));

    web::Json(clones)
}

#[post("/contacts")]
pub async fn add_contact(data : web::Data<AppState>, payload : web::Json<ContactDetails>) -> HttpResponse {
    data.contact_list.lock().unwrap().insert(&payload.0);
    HttpResponse::Ok().finish()
}

#[put("/contacts")]
pub async fn edit_contact(data : web::Data<AppState>, payload : web::Json<ContactDetails>) -> HttpResponse {
    data.contact_list.lock().unwrap().edit(&payload.0);
    HttpResponse::Ok().finish()
}

pub async fn delete_contact(data : web::Data<AppState>, contact_name: web::Path<String>) -> HttpResponse {
    data.contact_list.lock().unwrap().delete(&contact_name.into_inner());
    HttpResponse::Ok().finish()
}

//TODO: This is bad, but it works ¯\_(ツ)_/¯
//My service returns a Vec of borrowed objects.
//I can't just pass them to controllers::JSon for some reason, as it requires objects to be owned.. :(
//Figure out how to do this without cloning data every time.
fn clone(contacts : Vec<&ContactDetails>) -> Vec<ContactDetails> {
    contacts.iter().map(|c| ContactDetails {
        name : c.name.clone(),
        phone : c.phone.clone(),
        email : c.email.clone(),
    }).collect()
}