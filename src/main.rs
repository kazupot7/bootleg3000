use std::{error::Error, fmt::Debug, fs::File, io::{self, BufRead, BufReader, Write}};
use table::TableRow;

pub mod table;

#[derive(Debug)] 
struct CSVData {
    data: Vec<Vec<String>>,
    dimensions: Dimensions,
}

#[derive(Debug)] 
struct Dimensions {
    x: usize,
    y: usize,
}

struct BootLegEditor{
    csv_data: CSVData,
    file_path: String,
}

trait Editor {
    type Data;
    type Error;
    
    fn refresh_csv_data(file_path: &String, data: &Vec<Vec<String>>)-> Result<(), Self::Error> ;
    fn read_csv(file_path: &String) -> Result<Self::Data, Self::Error> ;
    fn display_csv(&mut self) -> Result<(), Self::Error>;
    fn display_paginated_csv(&mut self, limit: usize, page: usize) -> Result<(), Self::Error> ;
    fn modify_field(&mut self, row: usize, col: usize, value: String) -> Result<(), Self::Error>;
    fn delete_row(&mut self, row_index: usize) -> Result<(), Self::Error> ;
    fn delete_field(&mut self, row_index: usize, col_index: usize) -> Result<(), Self::Error> ;
}

impl BootLegEditor{
    fn new(file_path: String, csv_data: CSVData) -> Self {
        return Self{file_path, csv_data}
    }
    fn print_csv_as_table(&self, xa: usize, xb: usize) {
        for (index, data_row) in self.csv_data.data[xa..xb].iter().enumerate() {
            let serial_number = xa + index + 1;

            println!("{}", TableRow::new(serial_number, data_row));
        }
    }
}

impl Editor for BootLegEditor {
    type Data = CSVData;
    type Error = Box<dyn Error>; 

    fn read_csv(file_path: &String) -> Result<Self::Data, Self::Error> {
        let mut data: Vec<Vec<String>> = Vec::new();
    
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
    
        for line in reader.lines() {
            let record_values: Vec<String> = line?.split(',').map(String::from).collect();
            data.push(record_values);
        }
    
        let dimensions = Dimensions {
            x: data.len(),
            y: data[0].len(),
        };
    
        let result = CSVData{dimensions, data};
    
        Ok(result)
    }

    fn refresh_csv_data(file_path: &String, data: &Vec<Vec<String>>)-> Result<(), Self::Error>  {
        let mut file = File::create(file_path)?;
    
        for (idx , row) in data.iter().enumerate() {
            let mut line = row.join(",") ;
            if idx < data.len() - 1 {
                line += "\n"
            }
            file.write_all(line.as_bytes())?;
        }
    
        Ok(())
    }

    fn display_csv(&mut self) -> Result<(), Self::Error> {
        self.print_csv_as_table(0, self.csv_data.dimensions.x);
        Ok(())
    }

    // Display paginated CSV with limit and page
    fn display_paginated_csv(&mut self, limit: usize, page: usize) -> Result<(), Self::Error> {
        if page == 0 {
            return Err(Box::new(io::Error::new(io::ErrorKind::InvalidInput, "You've reached the top of the page.")));
        }
    
        let start_row = (page - 1) * limit;
        let mut end_row = start_row + limit;
        if end_row  >  self.csv_data.dimensions.x && start_row < self.csv_data.dimensions.x {
            end_row = self.csv_data.dimensions.x;
        }

        if end_row <= self.csv_data.dimensions.x {
            self.print_csv_as_table(start_row, end_row);
        } else {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "You've reached the bottom of the page.",
            )));
        }

        Ok(())
    }
    fn delete_row(&mut self, x: usize) -> Result<(), Self::Error> {
        if x <= self.csv_data.dimensions.x {
            self.csv_data.data.remove(x - 1);
            self.csv_data.dimensions.x -= 1;
        }

        _ = BootLegEditor::refresh_csv_data(&self.file_path, &self.csv_data.data)?;
        Ok(())
    }
    fn delete_field(&mut self, x: usize, y: usize) -> Result<(), Self::Error> {
        if x <= self.csv_data.dimensions.x && y <= self.csv_data.dimensions.y{
            for row in &mut self.csv_data.data {
                row.remove(y-1);
            }
            self.csv_data.dimensions.y -= 1;
        }
        
        _ = BootLegEditor::refresh_csv_data(&self.file_path, &self.csv_data.data)?;
        Ok(())
    }

    fn modify_field(&mut self, x: usize, y: usize, value: String)  -> Result<(), Self::Error>{
        if x <= self.csv_data.dimensions.x && y <= self.csv_data.dimensions.y {
            self.csv_data.data[x-1][y-1] = value;
        }

        _ = BootLegEditor::refresh_csv_data(&self.file_path, &self.csv_data.data)?;
        Ok(())
    }
}


fn main() {
    let file_path = "./testdata.csv".to_string();
    
    loop {
        let result = BootLegEditor::read_csv(&file_path);    
        match result {
            Ok(csv_data) => {
                    print_prompt();
                    let mut boot_leg_editor = BootLegEditor::new(file_path.clone(), csv_data);

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");

                    let input = input.trim().to_lowercase();
                    let parts: Vec<&str> = input.split_whitespace().collect();

                    match parts.len() {
                        0 => continue,
                        1 => {
                            match parts[0] {
                                "display" => boot_leg_editor.display_csv().unwrap_or_else(|error| eprintln!("Error: {:?}", error)),
                                "quit" | "exit" | "q" => break,
                                _ => eprintln!("[ERR] Unknown command: {}", parts[0]),
                            }
                        }
                        2 => {
                            let param: usize = match parts[1].parse() {
                                Ok(val) => val,
                                Err(_) => {
                                    eprintln!("[ERR] Invalid parameter: {}", parts[1]);
                                    continue;
                                }
                            };

                            match parts[0] {
                                "delete" => boot_leg_editor.delete_row(param).unwrap_or_else(|error| eprintln!("Error: {:?}", error)),
                                _ => eprintln!("[ERR] Unknown command: {}", parts[0]),
                            }
                        }
                        3 => {
                            let param1: usize = match parts[1].parse() {
                                Ok(val) => val,
                                Err(_) => {
                                    eprintln!("[ERR] Invalid parameter: {}", parts[1]);
                                    continue;
                                }
                            };
                            let param2: usize = match parts[2].parse() {
                                Ok(val) => val,
                                Err(_) => {
                                    println!("[ERR] Invalid parameter: {}", parts[2]);
                                    continue;
                                }
                            };

                            match parts[0] {
                                "modify" => {
                                    let mut value = String::new();
                                    print!("Please Enter the new value: ");
                                    io::stdout().flush().unwrap();
                                    io::stdin().read_line(&mut value).expect("Failed to read line");
                                    let value = value.trim().to_string();

                                    _ = boot_leg_editor.modify_field(param1, param2, value)
                                        .unwrap_or_else(|error| eprintln!("Error: {:?}", error));
                                }
                                "display" => {
                                    let mut page = param1;
                                    let limit = param2;

                                    let result = boot_leg_editor.display_paginated_csv(limit, page);
                                  
                                    match result {
                                        Ok(_) => {
                                            print!("Press 'n' for next page , 'b' to back page, 'q' to quit: ");
                                            loop {
                                                print!("> ");
                                                io::stdout().flush().unwrap();
                                                let mut input = String::new();
                                                io::stdin().read_line(&mut input).expect("Failed to read line");
        
                                                let input = input.trim().to_lowercase();
        
                                                match input.as_str() {
                                                    "n" => {
                                                        page += 1;
                                                        _ = boot_leg_editor.display_paginated_csv(limit, page)
                                                        .unwrap_or_else(|error| eprintln!("[ERR]: {:?}", error.to_string()))
                                                    }
                                                    "b" => {
                                                        page -= 1;
                                                        _ = boot_leg_editor.display_paginated_csv(limit, page)
                                                        .unwrap_or_else(|error| eprintln!("[ERR]: {:?}", error.to_string()))
                                                    }
                                                    "q" => break,
                                                    _ => {
                                                        eprintln!("[ERR] Invalid input");
                                                        continue;
                                                    }
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!("[ERR]: {:?}", e.to_string());
                                        }
                                    }
                                },
                                _ => eprintln!("[ERR] Unknown command: {}", parts[0]),
                            }
                        }
                        _ => eprintln!("[ERR] Invalid input"),
                    }
                }
                Err(e) => eprintln!("Unable to read CSV: {:?}", e),
            }
        }
}

fn print_prompt() {
    println!("BootLegEditor Prompt:");
    println!("1. DISPLAY  <page> <limit> - Display data where limit represents the limit and page represents the current page");
    println!("2. DELETE <row_index> - Delete row");
    println!("3. MODIFY <row_index> <col_index> - Modify field");
    println!("Type 'quit' or 'exit' to exit");
    print!("> ");
    io::stdout().flush().unwrap();
}