use bson::ordered::OrderedDocument;
use bson::{doc, Bson, Document};
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{error::Error, results::InsertOneResult, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub user_name: String,
    pub password: String,
    pub email: String,
}

#[derive(Clone)]
pub struct UserService {
    collection: Collection,
}

///
/// Build user from inputs
/// # Example :
///
/// ```
/// let user = build_user(
///     "hela",
///     "ben khalfallah",
///     "hela@hotmail.fr",
///     "helabenkhalfallah",
///     "azerty"
/// )
/// println!("user  = {:?}", user);
/// ```
fn build_user(
    id:String,
    email: String,
    user_name: String,
    password: String,
) -> User {
    User {
        id,
        user_name,
        password,
        email,
    }
}

///
/// Transform mongo db document to User
/// # Example :
///
/// ```
/// let cursor = self.collection.find(None, None).unwrap();
/// for result in cursor {
///    if let Ok(item) = result {
///      data.push(user_from_document(item))
///    }
/// }
/// ```
fn user_from_document(document: Document) -> User {
    let mut _id = "".to_string();
    let mut _email = "".to_string();
    let mut _user_name = "".to_string();
    let mut _password = "".to_string();

    if let Some(&Bson::String(ref id)) = document.get("_id") {
        _id = id.to_string();
    }
    if let Some(&Bson::String(ref email)) = document.get("email") {
        _email = email.to_string();
    }
    if let Some(&Bson::String(ref user_name)) = document.get("username") {
        _user_name = user_name.to_string();
    }
    if let Some(&Bson::String(ref password)) = document.get("password") {
        _password = password.to_string();
    }

    build_user( _id,_email, _user_name, _password)
}

/// Transform user to mongo db document
fn user_to_document(user: &User) -> Document {
    let User {
        id,
        user_name,
        password,
        email,
    } = user;
    doc! {
        "_id": id,
        "username": user_name,
        "password": password,
        "email": email,
    }
}

impl UserService {
    pub fn new(collection: Collection) -> UserService {
        UserService { collection }
    }

    /// Insert user in mongo db (user)
    pub fn create(&self, user: &User) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(user_to_document(user), None)
    }

    /// Update existing user in mongo db (email)
    pub fn update(&self, user: &User) -> Result<UpdateResult, Error> {
        let User {
            id,
            user_name: _user_name,
            password: _password,
            email:_email,
        } = user;
        self.collection
            .update_one(doc! { "_id": id}, user_to_document(user), None)
    }

    /// Delete existing user in mongo db (email)
    pub fn delete(&self, id: &String) -> Result<DeleteResult, Error> {
        self.collection.delete_one(doc! { "_id": id}, None)
    }

    /// get all users
    pub fn get(&self) -> Result<Vec<User>, Error> {
        let cursor = self.collection.find(None, None).unwrap();
        let mut data: Vec<User> = Vec::new();

        for result in cursor {
            if let Ok(item) = result {
                data.push(user_from_document(item))
            }
        }

        Ok(data)
    }

    /// Retrieve user by (id)
    pub fn get_user_id(&self, id: &String) -> Result<Option<OrderedDocument>, Error> {
        self.collection.find_one(doc! { "_id": id}, None)
    }
}
