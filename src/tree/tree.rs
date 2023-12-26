use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Tree {   
    pub name: String,
    pub locations: Vec<Point>
}

impl Tree {
    pub fn new() -> Tree {
        Tree {
            name: "tree".into(),
            locations: Vec::new()
        }
    }

    pub fn serialize(&self) -> std::io::Result<()> {
        let s = serde_json::to_string(self).unwrap();  
        fs::write(PATH, s)?;
        Ok(())
    }

    pub fn from_file(path: Option<&str>) -> Option<Tree> {
        let file = fs::read_to_string(path.unwrap_or(PATH)).unwrap();

        if file == "" {
            return None
        }

        Some(serde_json::from_str(file.as_str()).unwrap())

    }
}
