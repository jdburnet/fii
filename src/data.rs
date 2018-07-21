use std::fs::File;
use std::io;
use std::io::prelude::*;

use app_dirs::*;

pub const APP: AppInfo = AppInfo{ name: "fii", author: "na" };

pub fn save(f: &mut File, contents: &str) -> io::Result<()> {
    f.write_all(contents.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::env::temp_dir;
    use std::hash::{Hash, Hasher};
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
        let mut f = File::open(_path).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        contents == exp
    }

    #[test]
    fn save_saves() {
        let mut p = temp_dir();
        p.push(&APP.name);
        let mut file = File::create(p.as_path()).unwrap();
        let contents = time_seed().to_string();
        save(&mut file, &contents);
        assert!(contents_equal(p.to_str().unwrap(), &contents));
    }
}