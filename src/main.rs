use whimswardml::{is_section, whimsward};

fn main() {
    let this = whimsward("sunrise.whim");
    if is_section(this.as_str()) {
        println!("This file defines a Section.");
    } else {
        println!("{}",this);
    }
    
}
