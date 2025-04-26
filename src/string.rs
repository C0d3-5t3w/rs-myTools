pub trait StringExt {
    /// Check if a string is empty or only contains whitespace
    fn is_blank(&self) -> bool;
    
    /// Convert snake_case to camelCase
    fn to_camel_case(&self) -> String;
    
    /// Split a string into chunks of specified size
    fn chunks(&self, size: usize) -> Vec<String>;
}

impl StringExt for str {
    fn is_blank(&self) -> bool {
        self.trim().is_empty()
    }
    
    fn to_camel_case(&self) -> String {
        let mut result = String::new();
        let mut capitalize = false;
        
        for c in self.chars() {
            if c == '_' {
                capitalize = true;
            } else if capitalize {
                result.push(c.to_ascii_uppercase());
                capitalize = false;
            } else {
                result.push(c);
            }
        }
        
        result
    }
    
    fn chunks(&self, size: usize) -> Vec<String> {
        self.as_bytes()
            .chunks(size)
            .map(|chunk| String::from_utf8_lossy(chunk).to_string())
            .collect()
    }
}
