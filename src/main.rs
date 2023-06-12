use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("How to work with JSON in rust"),
        author: String::from("nyakio"),
        paragraph: vec![
            Paragraph {
                name: String::from("first sentense")
            },
            Paragraph {
                name: String::from("body of the paragraph ")
            },
            Paragraph {
                name: String::from("end of the paragraph")
            }
        ]
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The json is: {}", json);
}
