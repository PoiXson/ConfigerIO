
pub struct FileFinder {
	found: String,
}

impl FileFinder {

	pub fn new() -> Self {
		Self {
			found: "".to_string(),
		}
	}

	pub fn file(mut self, file: String) -> Self {
		if ! file.is_empty() {
			if self.found.is_empty() {
				if std::path::Path::new(&file).is_file() {
					self.found = file.clone();
				}
			}
		}
		self
	}

	pub fn found(self) -> String {
		self.found.clone()
	}

}
