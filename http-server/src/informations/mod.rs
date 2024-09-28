use names::Generator;
use serde::{Deserialize, Serialize};
use serde_json;
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
struct Information {
    name: String,
    age: u8,
}

#[derive(Serialize, Deserialize)]
struct InformationList {
    infos: Vec<Information>,
}

#[derive(Serialize, Deserialize)]
struct SingleObject {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct ObjectList {
    data: Vec<SingleObject>,
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

pub fn build_large_json() -> String {
    let mut object_list: ObjectList = ObjectList { data: Vec::new() };

    let mut generator = Generator::default();

    for _ in 0..1_00 {
        object_list.data.push(SingleObject {
            id: Ulid::new().to_string(),
            name: format!("{}", generator.next().unwrap()),
        });
    }

    match serde_json::to_value(&object_list) {
        Ok(object) => {
            format!("{}\n", object)
        }
        Err(e) => {
            format!("{}\n", e)
        }
    }
}
