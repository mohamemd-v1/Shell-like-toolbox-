use std::{ fs, path};
use colored::*;
use evalexpr::*;
use chrono::*;

use tar::{Archive};
use crate::backend::safe::Ugh;
use crate::backend::standard::tell;

use crate::backend::{clean::read_file_cont,safe::{ErrH, HyperkitError, Success, }};
use base64::{prelude::{BASE64_STANDARD, BASE64_STANDARD_NO_PAD, BASE64_URL_SAFE}, *};

pub fn calc (math:String) {
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


pub fn time() {
    let path= tell();

    let time = Local::now();
    println!("[{path:?}]~>[{time}]");
}


pub fn ship( ttype:String, flag:String , the_name_of_the_file:String , output_name:String) -> std::result::Result<() , HyperkitError> {
  use tar::{Builder};
  let path = tell();
    match ttype.trim() {
        "tar" => { 
            match flag.trim() {
                "--load" => {
                    let mut open = fs::File::open(&the_name_of_the_file).errh(Some("The file is not found".to_string())).ughf()?;
                    let make =  fs::File::create(format!("{}.tar", output_name)).errh(Some("Couldn`t make the file".to_string())).ughf()?;
                    let ifdir = open.metadata().errh(Some("The file is not found".to_string())).ughf()?;
                    if ifdir.is_dir() == true {
                        let mut builder1 = Builder::new(make);
                        let _apd = builder1.append_dir_all(&output_name, path::Path::new(&the_name_of_the_file)).errh(Some("Couldn't build the arcive".to_string())).ughf()?;
                        let _finsh = builder1.into_inner().errh(Some("Couldn't build the arcive".to_string()))._success_res("Ship completed", "loaded successfully").ughf()?;
                    }

                    else {
                        let mut builder2 = Builder::new(make);
                        let _ap = builder2.append_file(&output_name, &mut open).errh(Some("Couldn`t build the arcive".to_string())).ughf()?;
                        let _finsh = builder2.into_inner().errh(Some("Couldn't build the arcive".to_string()))._success_res("Ship completed", "loaded successfully").ughf()?;
                    }
                }
                "--Unload" => {
                    let open = fs::File::open(the_name_of_the_file).errh(Some("NotFound".to_string())).ughf()?;

                    let mut arc = Archive::new(open);
                    arc.unpack(output_name).errh(Some("Couldn't build the arcive".to_string()))._success_res("Ship completed", "Unloaded successfully").ughf()?;
                }   
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }
        }
        _ => {
            println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No type was supplied".red().bold());
        }
    }
    Ok(())
}

pub fn transmute (ttype:&str, flag:&str , the_name_of_the_file:&str , output_name:&str) -> std::result::Result<() , HyperkitError> {
    let path = tell();
    let readed = read_file_cont( the_name_of_the_file)?;
    match ttype.trim() {
        "base64-ST" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_STANDARD.encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = BASE64_STANDARD.decode(&readed.trim()).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }
        }
        "base64-PD" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_STANDARD_NO_PAD.encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = BASE64_STANDARD_NO_PAD.decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }
        }
        "base64-URL" => {
            match flag.trim() {
                "--enc" => {
                    let enc = BASE64_URL_SAFE.encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = BASE64_URL_SAFE.decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }            
        }
        "hex" => {
            match flag.trim() {
                "--enc" => {
                    let enc = hex::encode(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = hex::decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }
        }
        "HEX" => {
            match flag.trim() {
                "--enc" => {
                    let enc = hex::encode_upper(&readed);

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = hex::decode(&readed).unwrap_or_default();

                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }
        }
        "url" => {
            match flag.trim() {
                "--enc" => {
                    let enc = urlencoding::encode(&readed).into_owned();

                    fs::write(&output_name, enc).errh(Some("couldn`t write the encoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "encoded successfully").ughf()?;
                }
                "--dec" => {
                    let dec = urlencoding::decode(&readed).unwrap_or_default().into_owned();
            
                    fs::write(&output_name, dec).errh(Some("couldn`t write the decoded codic to the file Consider trying abother type".to_string()))._success_res("transmute" , "decoded successfully").ughf()?;
                }
                _ => {
                    println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag was supplied".red().bold());
                }
            }
        }
        _ => {
            println!("[{path:?}]~>{}: due to [{}]" , "Error".red().bold() , "No type was supplied".red().bold());
        }
    }
    Ok(())
}
