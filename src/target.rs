#[derive(Debug, Clone)]
pub struct Target {
    pub make: Option<String>,
    pub model: Option<String>,
}

pub struct TargetBuilder {
    make: Option<String>,
    model: Option<String>,
}

impl TargetBuilder {
    pub fn new() -> Self {
        TargetBuilder {
            make: None,
            model: None,
        }
    }

    pub fn make(&mut self, make: &str) -> &mut Self {
        self.make = Some(make.to_string());
        self
    }

    pub fn model(&mut self, model: &str) -> &mut Self {
        self.model = Some(model.to_string());
        self
    }

    pub fn build(self) -> Target {
        Target {
            make: self.make,
            model: self.model,
        }
    }
}
