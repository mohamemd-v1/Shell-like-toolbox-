use std::{ fs , path};

use colored::*;
use evalexpr::*;
use chrono::*;
use tar::{Archive};
use crate::backend::{safe::{SafeFile, SafeMeta, SafeVoid}, standard::tell};

//code:33
pub fn cul (math:String) {
    let path = tell();

    let e = match eval(&math) {
        Ok(t) => t,
        
        Err(error) => {
            println!("[{path:?}]>{}: due to {error:?}" , "Error".red() );
            return;
        }
    };

    println!("[{path:?}]~>[ \x1b[34m{e}\x1b[0m ]");
}

//cdoe:0
pub fn time() {
    let path= tell();

    let time = Local::now();
    println!("[{path:?}]~>[{time}]");
}

//code:1
pub fn ship( ttype:String, flag:String , the_name_of_the_file:String , output_name:String) -> std::io::Result<()> {
  use tar::{Builder};
  let path = tell();
    match ttype.trim() {
        "tar" => { 
            match flag.trim() {
                "--load" => {
                    let mut open = fs::File::open(&the_name_of_the_file).safe()?;
                    let make =  fs::File::create(format!("{}.tar", output_name)).safe()?;
                    let ifdir = open.metadata().safe()?;
                    if ifdir.is_dir() == true {
                        let mut builder1 = Builder::new(make);
                        let _apd = builder1.append_dir_all(&output_name, path::Path::new(&the_name_of_the_file)).safe();
                        let _finsh = builder1.into_inner().safe_mas("Ship completed".to_string() , "loaded successfully".to_string());
                    }

                    else {
                        let mut builder2 = Builder::new(make);
                        let _ap = builder2.append_file(&output_name, &mut open).safe();
                        let _finsh = builder2.into_inner().safe_mas("Ship completed".to_string(), "loaded successfully".to_string());
                    }
                }
                "--Unload" => {
                    let open = fs::File::open(the_name_of_the_file).safe()?;

                    let mut arc = Archive::new(open);
                    arc.unpack(output_name).safe_mas("Ship completed".to_string(), "Unloaded successfully".to_string())?;
                }   
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag were suplied".red().bold());
                }
            }
        }
        "gzip" => {
            match flag.trim() {
                "--load" => {}
                "--Unload" => {}
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Type were suplied".red().bold());
                }
            }
        }
        _ => {
            println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Type were suplied".red().bold());
        }
    }
    Ok(())
}
