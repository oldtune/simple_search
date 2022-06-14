use crate::error::Result;

pub struct Finder {
    pub file_path: String,
    pub file_name: String,
}

impl<'a> Finder {
    pub fn find(&self) -> Result<Vec<String>> {
        let files = self.find_files_in_folder(&self.file_path);
        let files = files.unwrap();
        let files_found = self.fuzzy_search_from_files(&self.file_name, &files);

        println!("{:?}", files_found);
        Ok(vec!["".to_string()])
    }

    fn find_files_in_folder(&self, path: &String) -> Result<Vec<String>> {
        let files = std::fs::read_dir(path)?;
        let files: Vec<String> = files
            .into_iter()
            .map(|f| f.unwrap().file_name().into_string().unwrap())
            .collect();
        Ok(files)
    }

    fn fuzzy_search_from_files(
        &self,
        file_name: &String,
        files: &'a Vec<String>,
    ) -> &'a Vec<String> {
        files
    }
}
