pub mod PRE {
    pub fn ExecuteBytecode(code: &str) -> String {
        let mut log: String = String::from("");
        
        let mut cmdt: String = String::from("");
        let mut cmdf: bool = false;
        
        println!("\x1b[00m");

        for token   in code.split_whitespace() {
            if token == "lg" {
                cmdf = false;
                cmdt = String::from("print");
            } else if token == "endl" {
                log.push_str("\n");
            } else if token == "col" {
                cmdf = false;
                cmdt = String::from("color");
            } else {
                if cmdt == "print" {
                    if !cmdf {
                      log.push_str(&(token.to_owned() + " "));
                    }
                } else if cmdt == "color" {
                    if !cmdf {
                        if token == "white" {
                            log.push_str("\x1b[00m");
                        } else if token == "black" {
                            log.push_str("\x1b[30m");
                        } else if token == "red" {
                            log.push_str("\x1b[31m");
                        } else if token == "green" {
                            log.push_str("\x1b[32m")
                        } else if token == "blue" {
                            log.push_str("\x1b[34m")
                        } else if token == "yellow" {
                            log.push_str("\x1b[33m")
                        }
                            cmdf = true;
                    }
                }
            }
        }
        log.push_str("\x1b[00m");
        return log
    }
}