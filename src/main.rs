enum Language {
    English,
    German
}

struct Greeter {
    language: Language,
}

impl Greeter {
    fn new() -> Greeter {
        Greeter { 
            language: Language::English,
         }
    }

    fn with_language(mut self, language: Language) -> Greeter {
        self.language = language;
        self
    }

    fn greet(self) {
        match self.language {
            Language::English => println!("Hello Rust"),
            Language::German => println!("Hallo Rust"),
        }
    }
}

fn main() {
    Greeter::new().with_language(Language::German).greet();
}



