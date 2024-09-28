use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Information {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
struct InformationList {
    infos: Vec<Information>,
}

pub fn get_json_response() -> String {
    let mut information_list: InformationList = InformationList {
        infos: vec![
            Information {
                name: "Alice".to_string(),
                age: 20,
            },
            Information {
                name: "Bob".to_string(),
                age: 25,
            },
            Information {
                name: "Charlie".to_string(),
                age: 30,
            },
        ],
    };

    information_list.infos.push(Information {
        name: "David".to_string(),
        age: 35,
    });

    match serde_json::to_value(&information_list) {
        Ok(object) => {
            format!("{}\n", object)
        }
        Err(e) => {
            format!("{}\n", e)
        }
    }
}
