// Make sure you add the `patch` crate to the `[dependencies]` key of your Cargo.toml file.
use patch::Patch;

pub struct Model {}

impl Model {
    pub fn new(dir: &str) -> Model {
        return Model {};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {

        // assert_eq!(Solution::count_time(String::from("?5:00")),2);
    }
}
