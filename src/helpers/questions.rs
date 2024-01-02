pub struct Questions {
    pub initial: String,
}

impl Questions {
    pub fn new() -> Self {
        Questions {
            initial: "What webserver are we building today?".to_string(),
        }
    }
}
