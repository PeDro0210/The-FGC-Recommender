use actix_web::web::Json;
use crate::api::queries_structs_endpoint::*;


// fn start_driver() -> neo4rs::Graph  { //returns the driver
//     //TODO: Create a connection to the database
    
// }

// creates a user node in the database that is has a Like relationship with the categories
async fn user_node_creation(categories:Vec<String>) -> User {
    // TODO: Implement the logic to create a user node in the database with the categories
    return User{name: "user_name".to_string()};
}


// Makes a query for content-based filtering using jaccard similarity with the created user node
// TODO: see if Json works out of the box
async fn jaccard_similarity_query() -> Json<Game> { //returns the games
    // TODO: Implement the query logic here
    return Json(Game{name: "game_name".to_string(), image_url: "image_url".to_string()});
}

// Fetches the questions from the database
async fn get_questions() -> Json<Questions> { //returns the questions
// TODO: Fetch the questions from the node Questions with there answears and category for each answears.
    return Json(Questions{list_of_quetions: vec!["question1".to_string(), "question2".to_string()], list_of_answers: vec!["answer1".to_string(), "answer2".to_string()]});
}

// Fetches the characters from the DB based on the game name and archetype
async fn get_characters_from_game(game_name: String, arhetype: Vec<String>) -> Json<Character> { //returns the characters
// TODO: Fetch the characters from the DB based on the game name and archetype
    return Json(Character{name: "character_name".to_string(), image_url: "image_url".to_string()});
}