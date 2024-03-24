mod cxx;

#[derive(Default)]
pub struct RustType {
    pub value: i32,
}

impl RustType {
    fn inc(&mut self) {
        self.value += 1;
    }

    fn show(&self) {
        println!("current value is {}", self.value);
    }
}

pub fn concat_two_strings(a: &str, b: &str) -> String {
    a.to_string() + b
}

mod tests {
    #[test]
    fn test_concat_two_strings() {
        assert_eq!(super::concat_two_strings("1", "2"), "12".to_string());
    }
}
