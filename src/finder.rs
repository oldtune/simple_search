use crate::error::Result;

pub struct Finder {
    pub file_path: String,
    pub file_name: String,
}

impl Finder {
    pub fn find(&self) -> Result<Vec<String>> {
        Ok(vec!["".to_string()])
    }

    fn find_in_folder(path: &String) -> Result<Vec<String>> {
        let files = std::fs::read_dir(path)?;
        let files: Vec<String> = files
            .into_iter()
            .map(|f| f.unwrap().path().display().to_string())
            .collect();
        Ok(files)
    }
}
