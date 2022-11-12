use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("This is an article"),
        author: String::from("John Doe"),
        paragraph: vec![
            Paragraph {
                name: String::from("This is a paragraph"),
            },
            Paragraph {
                name: String::from("This is another paragraph"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();

    println!("the json is: {}", json);
}
