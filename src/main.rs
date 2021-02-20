use std::{
    path::PathBuf,
    env,
    fs::{self, File},
    collections::HashMap,
};

fn main() {
    let help_str = r#"
set <key> <value>: add a key & value to db
get <key>: get the value for a ey from db [case sensitive]
rm <key>: remove a key from db [case sensitive]
"#;

    let mut args = env::args().skip(1);
    let command = args.next().expect("No command found!");
    let k = args.next().expect("No key found!");
    
    let mut db = Database::new("kv.db").unwrap();

    if command == "set" {
        let v = args.next().expect("No value found");
        db.put(&k, &v);
    } else if command == "get" {
        if let Some(v) = db.get(&k) {
            println!("{}:{}", k, v);
        } else {
            println!("Oopsie, couldn't find '{}' in db", k);
        }
    } else if command == "rm" {
        if let Some(v) = db.remove(&k) {
            println!("Removed {}:{} from db", k, v);
        } else {
            println!("Couldn't find '{}' in db", k);
        }
    } else{
        eprintln!("I DON'T KNOW WHAT YOU MEAN!\n{}", help_str);
    }

}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new(path: &str) -> Result<Self, std::io::Error> {
        let mut map = HashMap::new();
        
        let path = PathBuf::from(path);

        if path.is_file() {
            let contents = fs::read_to_string(path)?;
 
            for line in contents.lines() {
                let mut chunks = line.splitn(2, '\t');
                let k = chunks.next().expect("No Key!");
                let v = chunks.next().expect("No Value!");

                map.insert(k.to_owned(), v.to_owned());
            }
        } else {
            File::create(path)?;
        }

        Ok(Self { map })
    }
}

impl Database {
    fn put(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_owned(), value.to_owned());
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }

    fn remove(&mut self, key: &str) -> Option<String> {
        self.map.remove(key)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        let mut contents = String::new();

        for (k, v) in &self.map {
            contents.push_str(&k);
            contents.push('\t');
            contents.push_str(&v);
            contents.push('\n');
        }

        fs::write("kv.db", contents).expect("Failed to write to db");
    }
}
