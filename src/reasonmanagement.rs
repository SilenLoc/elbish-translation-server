pub struct Reason {
    reason: String,
    identifier: String,
    solution: String,
}

impl Reason {
    pub fn new(reason: &str, identifier: &str, solution: &str) -> Self {
        Reason {
            reason: reason.to_string(),
            identifier: identifier.to_string(),
            solution: solution.to_string(),
        }
    }

    pub fn build(self) -> String {
        self.reason + " -> " + &self.identifier + " <- " + "solution: " + &self.solution
    }
}
