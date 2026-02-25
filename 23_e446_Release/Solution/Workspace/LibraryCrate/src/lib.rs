pub mod mod1;
pub mod mod2;
pub fn lib_function() -> String {
    "Hello from the LibraryCrate library!".to_string()
}

pub mod mod3 {
    
    pub fn mod3inline_function() -> String {
        "Hello from mod3! inline".to_string()
    }

    pub mod mod3sub1 {
        pub fn mod3sub1inline_function() -> String {
            "Hello from mod3sub1! inline".to_string()
        }
    }
}

pub mod mod4 {
    pub fn mod4inline_function() -> String {
        "Hello from mod4! inline".to_string()
    }
}