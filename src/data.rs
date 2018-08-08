use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;

use app_dirs::*;

pub const APP: AppInfo = AppInfo{ name: "fii", author: "na" };

pub fn save(f: &mut File, contents: &str) -> io::Result<()> {
    f.write_all(contents.as_bytes())?;
    Ok(())
}

pub fn load(_path: &str) -> io::Result<String> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::env::temp_dir;
    use std::hash::{Hash, Hasher};
    use std::path::PathBuf;
    use std::time::SystemTime;

    use app_dirs::*;

    use super::*;

    #[test]
    fn get_app_name() {
        APP.name;
    }

    #[test]
    fn app_data_root_has_app_name() {
        let d = get_app_root(AppDataType::UserData, &APP).unwrap();
        let s = d.to_str().unwrap();
        assert!(s.contains(&APP.name));
    }

    fn time_seed() -> u64 {
        let now = SystemTime::now();
        let mut hasher = DefaultHasher::new();
        now.hash(&mut hasher);
        hasher.finish()
    }

    fn contents_equal(_path: &str, exp: &str) -> bool {
        let contents = load(_path).unwrap();
        contents == exp
    }

    fn test_file(_path: &mut PathBuf) -> File {
        _path.push(&time_seed().to_string());
        File::create(_path.as_path()).unwrap()
    }

    #[test]
    fn save_saves() {
        let mut p = temp_dir();
        let mut file = test_file(&mut p);
        let contents = time_seed().to_string();
        save(&mut file, &contents).unwrap();
        assert!(contents_equal(p.to_str().unwrap(), &contents));
    }

    #[test]
    fn load_loads() {
        let mut p = temp_dir();
        let mut file = test_file(&mut p);
        let exp = time_seed().to_string();
        save(&mut file, &exp).unwrap();
        let contents = load(p.to_str().unwrap());
        assert_eq!(exp, contents.unwrap());
    }
}
