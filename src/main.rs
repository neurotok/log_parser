use std::env;
use csv::Writer;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::io::prelude::*;

fn save_work(r: Vec<(String, String,  String)>) -> Result<(), Box<dyn Error>> {

    let mut ff = fs::File::create("failed.txt")?;

    let mut wtr = Writer::from_path("report.csv")?;
    
    wtr.write_record(&["Failed test", "Logged cause of error", "Optional validation errors and others and other logged information "])?;

    for record in r {
        wtr.write_record(&[record.0.as_str(), record.1.as_str(), record.2.as_str()])?;

        let mut test_name = record.0;
        test_name.push_str("\n");
        ff.write_all(test_name.as_bytes()).unwrap();
    }

    wtr.flush()?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let f = fs::File::open(&args[1]).unwrap();
    let mut buffer = Vec::new();
    let f = std::io::BufReader::new(f);
    for line in f.lines() {
        buffer.push(line.unwrap());
    }

    let mut case: Vec<String> = Vec::new();
    let mut cases: Vec<Vec<String>> = Vec::new();

    for line in buffer {
        if !line.is_empty() {
            case.push(line);
        } else {
            cases.push(case.clone());
            case.clear();
        }
    }

    println!("Found: {} cases", cases.len());

    let case_name_re = Regex::new(r"Test case '(.*)'").unwrap();
    let fail_couse_re = Regex::new(r"Fail \((.*?)\)$").unwrap();

    let mut parsed_cases: Vec<(String, String, String)> = Vec::new();

    for case in cases {
        let mut case_details: (String, String, String) =
            (String::new(), String::new(), String::new());
        for line in case {
            //Capture case name
            if let Some(c) = case_name_re.captures(line.as_str()) {
                let name = c.get(1).map_or("", |m| m.as_str());
                case_details.0 = String::from(name);
            }
            //Capture cause of error
            else if let Some(c) = fail_couse_re.captures(line.as_str()) {
                let couse = c.get(1).map_or("", |m| m.as_str());
                case_details.1 = String::from(couse);
            }
            //Capture optional validation errors
            else {
                if case_details.2.is_empty() {
                    case_details.2.push_str(line.as_str());
                } else {
                    //Append another validation code
                    case_details.2.push_str("\n");
                    case_details.2.push_str(line.as_str());
                }
            }
        }
        //Push only if error code present
        if !case_details.1.is_empty() {
            parsed_cases.push(case_details);
        }
    }

    println!("Found: {} failing tests", parsed_cases.len());

    save_work(parsed_cases).expect("Error writing to file");
}
