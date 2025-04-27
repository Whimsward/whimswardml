use std::{
    fs,
    io,
};

const SILCROW : &str = "§";

const WHIM : &str = "src/whim/";

/// Opens a document type declaration a la html
const DOCTAG : &str = "<!WHIMDOC";


/// Closes a tag
const CLOSE_TAG : &str = ">";


const SEMICOLON : &str = ";";

pub trait Declaration {
    fn decl(text: &str) -> Decl;
}

pub struct DocDecl {
    name : String,
}

impl Declaration for DocDecl {
    fn decl(text: &str) -> Decl {
        let full_decl = format!(
            "{} {}{}",
            DOCTAG,
            text,
            CLOSE_TAG
        );
        DocDeclaration(full_decl)
    }
}


pub struct SectionDecl {
    name : String,
}

impl Declaration for SectionDecl {
    fn decl(text: &str) -> Decl {
        let my_name = String::from(text);
        Decl::SectionDeclaration(my_name)
    }
}

pub struct NullDecl {
    null_string : String,
}

impl Declaration for NullDecl {
    fn decl(text: &str) -> Decl {
        let not_a_declaration = String::from(text);
        let my_decl = NullDecl {null_string: not_a_declaration.clone()};
        let null_notice = String::from("This text is not a Declaration:");
        let me = format!("{}\n {}",null_notice,not_a_declaration);
        println!("{}",me);
        Decl::NullDeclaration(my_decl.null_string)
    }
}

#[derive (Debug,PartialEq)]
pub enum Decl {
    DocDeclaration(String),
    SectionDeclaration(String),
    NullDeclaration(String),
}


/// Enumerates the types of Whimsward Document supported by the parser
#[derive (Debug)]
pub enum WhimsDoc {
    Canon(String),
    Narrative(String),
    Sequence(String),
    Character(String),
    Artifact(String),
}

pub fn get_doc_type(whim_doc : String) -> WhimsDoc {
    let lines = whim_doc.lines();
    if 
    
}

/// Checks if a line starts with a Silcrow to indicate a section
pub fn is_section(line: &str) -> bool {
    if line.starts_with(SILCROW) {
        true
    } else {false}
}

/// Checks if a line starts with DOCTAG.
pub fn is_whimdoc(file_contents : &str) -> bool {
    if file_contents.starts_with(DOCTAG) {
         true
    } else {false}
}


pub fn whimsward(file_name: &str) -> String {
    let full_path = WHIM.to_owned() + file_name;
    let result = fs::read_to_string(full_path);
    
    if is_whimdoc(result.unwrap()) {
        result.unwrap()
    } else { 
        String::from("This is not a Whimsward document.")
    }

    match result {
        Ok(content) => content,
        Err(e_message) => e_message.to_string(),
    }
}



pub fn parse_line(text: &str) -> Decl {
    if is_section(text) {
        SectionDecl::decl(text)
    } else {
        NullDecl::decl(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_section() {
        let a_section_decl = format!("{} That Time I Got Really Sleepy",SILCROW);
        
        let parsed_section = parse_line(&a_section_decl.as_str());
        
        println!("Is the following line a Section Declaration? \n {}",a_section_decl);
        let a_raw_decl = Decl::SectionDeclaration(a_section_decl);
        assert_eq!(a_raw_decl,parsed_section);
    }

    #[test]
    fn test_parse_null() {
        let not_a_declaration = String::from("This text is not a Declaration.");
        let parsed_non_decl = parse_line(&not_a_declaration.as_str());
        println!("Is the following line a Null Declaration? \n {}",not_a_declaration);
        let a_raw_null = Decl::NullDeclaration(not_a_declaration);
        assert_eq!(a_raw_null,parsed_non_decl);
    }

    #[test]
    fn test_parse_many() {
        let test_whim = whimsward("sunrise.whim");
        for line in test_whim.lines() {
            let test_line = parse_line(line);
            match test_line {
                Decl::SectionDeclaration(this_text) => println!("{}",this_text),
                Decl::NullDeclaration(that_text) => println!("{}",that_text),
            };
        }
    }
}

