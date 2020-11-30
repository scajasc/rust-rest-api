use bson::ordered::OrderedDocument;
use bson::{doc, Bson, Document};
use mongodb::results::{DeleteResult, UpdateResult};
use mongodb::{error::Error, results::InsertOneResult, Collection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub user_id: String,
    pub todo: String,
}

#[derive(Clone)]
pub struct TaskService {
    collection: Collection,
}

/// Build user from inputs
fn build_task(
    id:String,
    user_id: String,
    title: String,
    description: String,
    todo:String,
) -> Task {
    Task {
        id,
        title,
        description,
        user_id,
        todo,
    }
}


/// Transform mongo db document to User
fn task_from_document(document: Document) -> Task {
    let mut _id = "".to_string();
    let mut _user_id = "".to_string();
    let mut _title = "".to_string();
    let mut _description = "".to_string();
    let mut _todo = "".to_string();

    if let Some(&Bson::String(ref id)) = document.get("_id") {
        _id = id.to_string();
    }
    if let Some(&Bson::String(ref user_id)) = document.get("user_id") {
        _user_id = user_id.to_string();
    }
    if let Some(&Bson::String(ref title)) = document.get("title") {
        _title = title.to_string();
    }
    if let Some(&Bson::String(ref description)) = document.get("description") {
        _description = description.to_string();
    }
    if let Some(&Bson::String(ref todo)) = document.get("todo") {
        _todo = todo.to_string();
    }

    build_task( _id,_user_id, _title, _description, _todo)
}

/// Transform user to mongo db document
fn task_to_document(task: &Task) -> Document {
    let Task {
        id,
        title,
        description,
        user_id,
        todo,
    } = task;
    doc! {
        "_id": id,
        "title": title,
        "description": description,
        "user_id": user_id,
        "todo": todo,
    }
}

impl TaskService {
    pub fn new(collection: Collection) -> TaskService {
        TaskService { collection }
    }

    /// Insert task in mongo db (user)
    pub fn create(&self, task: &Task) -> Result<InsertOneResult, Error> {
        self.collection.insert_one(task_to_document(task), None)
    }

    /// Update existing task in mongo db (user_id)
    pub fn update(&self, task: &Task) -> Result<UpdateResult, Error> {
        let Task {
            id,
            title: _title,
            description: _description,
            user_id:_user_id,
            todo:_todo,
        } = task;
        self.collection
            .update_one(doc! { "_id": id}, task_to_document(task), None)
    }

    /// Delete existing task in mongo db (user_id)
    pub fn delete(&self, id: &String) -> Result<DeleteResult, Error> {
        self.collection.delete_one(doc! { "_id": id}, None)
    }

    /// get all task
    pub fn get(&self) -> Result<Vec<Task>, Error> {
        let cursor = self.collection.find(None, None).unwrap();
        let mut data: Vec<Task> = Vec::new();

        for result in cursor {
            if let Ok(item) = result {
                data.push(task_from_document(item))
            }
        }

        Ok(data)
    }

    /// Retrieve task by (id)
    pub fn get_task_id(&self, id: &String) -> Result<Option<OrderedDocument>, Error> {
        self.collection.find_one(doc! { "_id": id}, None)
    }
}
