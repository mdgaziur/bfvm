pub struct ErrorCtx {
    source: String
}

impl ErrorCtx {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string()
        }
    }

    pub fn raise(&self, message: &str, position: usize){
        println!("{}", message);

        let slice_of_source = {
            if self.source.len() > 10 {
                &self.source[position-9..position]
            } else {
                &self.source[..position]
            }
        };

        println!("{}", slice_of_source);
        println!("{}^ Here", " ".repeat(slice_of_source.len()));
    }
}