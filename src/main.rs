use regex::Regex;
use std::fs;
use std::io::prelude::*;

fn main() {
    let mut f = fs::File::open("test.txt").unwrap();
    let mut buffer = String::new();

    //Read all
    f.read_to_string(&mut buffer).unwrap();

    //Split by empty lines
    let cases: Vec<&str> = buffer.split("\n\r").collect();

    let case_name_re = Regex::new(r"Test case '(.*)'").unwrap();
    let validation_error_name_re = Regex::new(r"validateFunctionArguments, (.*)'").unwrap();
    let fail_couse_re = Regex::new(r"Fail \((.*?)\)$").unwrap();

    let mut parsed_cases: Vec<(String, String, String)> = Vec::new();

    for case in cases {
        let mut case_details: (String, String, String) = (String::new(), String::new(), String::new());
        for line in case.lines() {
            //Capture case name
            if let Some(c) = case_name_re.captures(line) {
                let name= c.get(1).map_or("", |m| m.as_str());
                case_details.0 = String::from(name);
            }
            //Capture optional validation errors
            if let Some(c) = validation_error_name_re.captures(line) {
                //First validation code
                if case_details.1.is_empty(){
                    let error = c.get(1).map_or("", |m| m.as_str());
                    case_details.1 = String::from(error);
                }
                else {
                    //Append another validation code
                    let error = c.get(1).map_or("", |m| m.as_str());
                    case_details.1.push_str("\n");    
                    case_details.1.push_str(error);    
                }
            }
            //Capture code name
            if let Some(c) = fail_couse_re.captures(line) {
                let couse = c.get(1).map_or("", |m| m.as_str());
                case_details.2 = String::from(couse);
            }
        }
        //Push only if error code present
        if !case_details.2.is_empty() {
            parsed_cases.push(case_details);
        }
    }

    for case in parsed_cases {
        println!("{}\n{}\n{}\n", case.0, case.1, case.2);
    }

}
