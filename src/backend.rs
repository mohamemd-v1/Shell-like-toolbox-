pub mod commands {
use colored::Colorize;
use sysinfo::{System};
pub const GITHUBLINK:&str = "https://github.com/mohamemd-v1/Shell-like-toolbox-.git";
use nix::{sys::{self, signal::{*}}, unistd::Pid};
use reqwest::blocking;
use crate::backend::{safe::{ Safe, SafeT}, standard::{input, tell}};
use std::{env::{self, }, fs::{self,File}, io::*,  path::PathBuf  , process};
    pub fn help(helpt:String) {
       match helpt.trim() {
        "--commands" => {
            println!("   *{} {} to end the session" , "enter".green() , "end".bright_blue() );
            println!("   *{} {} to clear the screen" , "enter".green() , "clean".bright_blue() );
            println!("   *{} {} {} to change the dir" , "enter".green() , "go".bright_blue(), "<Dir>".bright_purple() );
            println!("   *{} {} to see the current dir" , "enter".green() , "wh".bright_blue() );
            println!("   *{} {} to show all the files in the current dir" , "enter".green() , "see".bright_blue() );
            println!("   *{} {} {} to see what is inside the file" , "enter".green() , "peek ".bright_blue() ,  "<Path>".bright_purple() );
            println!("   *{} {} {} to delete anything" , "enter".green() , "burn".bright_blue() , "<Path/File>".bright_purple());
            println!("   *{} {} {} {} to copy a file" , "enter".green() , "clone".bright_blue() , "<Name/File>".bright_purple() , "<Nname/File>".bright_yellow());
            println!("   *{} {} {} to create a file" , "enter".green() , "forge".bright_blue() , "<Name>".bright_purple());
            println!("   *{} {} {} to make a dir" , "enter".green() , "mk".bright_blue() , "<Name>".bright_purple());
            println!("   *{} {} {} to run a program" , "enter".green() , "run".bright_blue() , "<App>".bright_purple() );
            println!("   *{} {} {} to move a file from place to another" , "enter".green() , "mv".bright_blue() , "<Name>".bright_purple());
            println!("   *{} {} {} to find the dir of a file" , "enter".green() , "find".bright_blue() , "<FileName>".bright_purple());
            println!("   *{} {} {} to list and lookup prosses" , "enter".green() , "ps".bright_blue() , "<Flag[-SF: search for pros , -A: list all the pros]>".bright_purple());
            println!("   *{} {} {} to stop prosses |{}|" ,  "enter".green() , "stop".bright_blue() , "<PID>".bright_purple() , "#Warning do not even attempt to enter latters only numbers is allowed otherwise it will stop itself!!".bright_red().bold());
            println!("   *{} {} {} to ping servers or websites to check if they are up" , "enter".green() , "call".bright_blue() , "<IP|URL>".bright_purple() );
        }
        "--built-in-apps" => {
            println!("   *{} {} {} to use the built-in calculator" , "enter".green() , "calc".bright_blue() , "<Math>".purple());
            println!("   *{} {} to know the time" , "enter".green() , "time".bright_blue() );
            println!("   *{} {} {} {} {} {} to make/extract tar files" , "enter".green() , "ship".bright_blue() , "<Type>".bright_purple(), "<Flag>".bright_yellow() , "<File-Name>".bright_cyan() , "<File-Outpot-Name>".bright_magenta());
            println!("   *{} {} {} {} {} {} to encode/decode files" , "enter".green() , "transmute".bright_blue() , "<Type>".bright_purple(), "<Flag>".bright_yellow() , "<File-Name>".bright_cyan() , "<File-Outpot-Name>".bright_magenta());
            println!("   *{} {} {} to edit and read files or text" , "enter".green() , "vortex".bright_blue() , "<File>".bright_purple());
        }
        "--about" => {
            println!("{}HyperKit is a modern, extensible, and lightweight command-line environment built to unify the tools you need into one powerful workspace." , "@".bright_green() )
        }
        "ship" => {
                println!("   *[{}: tar][{}: --load {} --Unload {}]" , "Types".bright_green().bold() , "flags".bright_blue().bold(), "to make an arcive".bright_purple().bold() , "to extract an arcive".bright_yellow().bold());
        }
        "transmute" => {
                println!("   *[{}: base64-PD<pedding> , base64-ST<standerd> , base64-URL<url> , hex<low-case hex> , Hex<uper-case hex> ][{}: --enc {} --dec {}]" , "Types".bright_green().bold() , "flags".bright_blue().bold(), "to encode a file".bright_purple().bold() , "to decode a file".bright_yellow().bold());
        }
        "vortex" => {
            println!("  *[{}: {}<To save the file or edits> {}<To quit without saving anything>]" , "codes".bright_green().bold() , "CTRL+s".bright_blue().bold() , "CTRL+q".bright_purple().bold());
        }
        _ => {
            println!("   *{} {} {} to see all the commands , {} to list all the available built in apps , {} for about" , "Enter".green()  , "help".red() ,"--commands".bright_purple() , "--built-in-apps".bright_purple() , "--about".bright_purple() );
         } 
       }    
    }

   
    pub fn clean() -> std::io::Result<()> {
       print!("\x1B[2J\x1B[1;1H");
       stdout().flush().safe(Some(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str()));
       Ok(())
    }

    
    pub fn go(t:String) -> std::io::Result<()> { 
        let path = PathBuf::from(&t);
        env::set_current_dir(&path).safe_mas("Go" , "directory has been changed successfully", Some(&t));
        Ok(())
    }
    
  
    pub fn  wh() -> std::io::Result<()> {
        let path = tell();

        let wh = env::current_dir().safe_w_res(Some(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str()))?;
        println!("[{path:?}]~>{}\x1b[34m{}\x1b[0m" ,"~".bright_green(), wh.display());
        Ok(())
    }

    
    pub fn see () -> std::io::Result<()> {
        let path = tell();

        let cur = env::current_dir().safe_w_res(None)?;
        let dir = fs::read_dir(cur);

        match dir {
            Ok(w) => {
                for i in w {
                    let dir = match i {Ok(t) => t, Err(e) => {println!("[{path:?}]>Error: due to {e:?}"); return Ok(());}};
                    println!("   {}\x1B[94m{}\x1b[0m" ,"~".bright_green() , dir.file_name().to_string_lossy());
                } 
            }
            Err(error) => {
                println!("[{path:?}]~>{}: due to \x1b[33m{error:?}\x1b[0m" , "Error".red());
                return Ok(());
            }
        }
        Ok(())
    }

   
    pub fn peek(file:String) -> std::io::Result<()> {
        let path = tell();
        let fe = File::open(&file);

        if let Err(e) = &fe {
            if e.kind() == ErrorKind::NotFound {
                println!("[{path:?}]~>{}: couldn't open the file due to [{}]" , "Error".red().bold() , "NotFound error".red().bold());
                println!("[{path:?}]~>Do you want to make this file?");
                print!("[{path:?}]~>({}/{}):" , "Y".green() , "N".red());
                stdout().flush().safe(Some(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str()));

                let yesorno = input();

                if yesorno == "Y" {
                    fs::File::create(&file).safe(Some(&file));
                }
            }
        };

        let fe = &mut fe?;

        let mut r = String::new();
        let _read =  fe.read_to_string(&mut r).safe(&file);

        println!("\x1b[34m{}\x1b[0m" , r);
        Ok(())
    }

    
    pub fn mk(path:String) -> std::io::Result<()> {
        fs::create_dir(&path).safe_mas("Mk", "Directory created successfully" , Some(&path));
        Ok(())
    }

    
    pub fn burn(path:String) -> std::io::Result<()> {
        let tell = tell();

        let burn = fs::remove_file(&path);

        if burn.is_ok() == true {
                println!("[{path:?}]~>{}: [{}]" , "burn".bright_green().bold() , "file has been burned successfully".bright_green().bold());
            }

        if let Err(e) = burn {
            match e.kind() {
                ErrorKind::IsADirectory => {
                    let burn_dir = fs::remove_dir(&path);

                    if let Err(e) = burn_dir {
                        if e.kind() == ErrorKind::DirectoryNotEmpty {

                            print!("[{tell:?}]~>[{}/{}]: the Directory is Not Empty do you stil want to delete it? >> " , "Y".bold().green() , "N".bold().red());
                            stdout().flush().safe(Some(format!("code:404 , this error shouldn`t occuer , report it to {}" , GITHUBLINK).as_str()));

                            let yesorno = input();
                            if yesorno == "Y" {
                                fs::remove_dir_all(&path).safe_mas("burn", "Directory has been burned successfully", Some(&path));
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn rn(f:String , t:String) -> std::io::Result<()> {
        fs::rename(&f, &t).safe_mas("rn", "Renamed successfully",Some(format!("{}+{}" , &f , &t).as_str()));
        Ok(())
    }

    pub fn clone(f:String , t:String) -> std::io::Result<()> {
        fs::copy(&f, &t).safe_mas("clone", "Copied!", format!("{}+{}" , &f , &t).as_str());
        Ok(())
    }

    pub fn forge(file:String) -> std::io::Result<()> {
        fs::File::create(&file).safe_mas("Forge completed!", "File created" , Some(&file));
        Ok(())
    }

    pub fn run(app:String) -> std::io::Result<()> {
        let path = tell();
        let run = process::Command::new(&app).output().safe_w_res(Some(&app))?;

        println!("[{path:?}]~>\x1b[34m{}\x1b[0m" , String::from_utf8_lossy(&run.stdout));
        Ok(())
    }

   
    pub fn mv(name:String , path:String) -> std::io::Result<()> {
        fs::copy(&name, format!("{}/{}" , &path , &name)).safe(format!("{}/{}" , &path, &name).as_str());

        let delete_eveadnice = fs::remove_file(&name);

        if let Err(e) = delete_eveadnice {
            match e.kind() {
                ErrorKind::IsADirectory => {
                    fs::remove_dir_all(&name).safe_mas("mv", "moving completed" , Some(format!("{}/{}" , &path, &name).as_str()));
                }
                _ => {}
            }
        }

        Ok(())
    }

    pub fn find(file_path:&str) -> std::io::Result<()> {
        use walkdir::*;
        let tell = tell();
        let mut err = false;

        let find = WalkDir::new("/").into_iter().filter_map(|e| e.ok());

        for i in find {
            if i.file_name() == file_path {
                println!("[{tell:?}]~> [{}] {}: \x1b[33m{}\x1b[0m", "find".bright_green().bold(), "found at".bright_green().bold(), i.path().display() );
                err = true;
            }
        }

        if err == false {
            println!("[{tell:?}]~> [{}] {}: \x1b[31m{}\x1b[0m", "find".bright_green().bold(), "Couldn`t find it anywhere".bright_red().bold(), &file_path);
        }

        Ok(())
    }

    pub fn ps(_flag:&str , _pid:usize) -> std::io::Result<()> {
        use sysinfo::Pid;
        
        let tell = tell();
        let mut sys = System::new_all();
        sys.refresh_all();
        match _flag {
            "-SF" => {
                if _flag == "-SF" {
                        if let Some(p) = sys.process(Pid::from(_pid)) {
                            println!("[{tell:?}]~>[{}] \x1B[1m\x1B[36m{}\x1B[0m\x1B[0m | {}:\x1B[1m\x1B[32m{}\x1B[0m\x1B[0m Gib | \x1B[1m\x1B[36m{}\x1B[0m\x1B[0m:\x1B[1m\x1B[32m{}\x1B[0m\x1B[0m Gib" ,"ps".bright_green().bold(), p.name().display() , "Disk usage".bright_yellow().bold() ,p.disk_usage().total_written_bytes as f64 / f64::from(1024).powi(3) , "memory usage".bright_yellow().bold() ,p.memory() as f64  / f64::from(1024).powi(3));
                    }
                        if let None = sys.process(Pid::from(_pid)) {
                            println!("[{tell:?}]~>[{}] {}: process not found or not running \x1b[1m\x1b[31m<{}>\x1b[0m\x1b[0m" , "ps".bright_green().bold() , "Error".bright_red().bold() , _pid )
                        }
                }
            }
            "-A" => {
                for (pid , pros) in sys.processes() {
                    println!("[\x1B[1m\x1B[32m{pid}\x1B[0m\x1B[0m]~>\x1B[1m\x1B[36m{}\x1B[0m\x1B[0m" , pros.name().display())
                }
            }
            _ => {
                println!("[{tell:?}]~>{}: due to [{}]" , "Error".red().bold() , "No Flag were suplied".red().bold());
            }
        }
        Ok(())
    }
    
    pub fn stop(pid:i32) {
        let tell = tell();

        let pid = Pid::from_raw(pid);

        let _kill = match sys::signal::kill(pid, SIGKILL) {
            Ok(_) => println!("[{tell:?}]~>{}: [{}]" , "stop".bright_green().bold() , "the target has been stoped!".bright_green().bold()),
            Err(e) => {
                println!("[{tell:?}]~>{}: due to \x1b[1m[\x1b[31m{e}]\x1b[0m\x1b[0m" , "Error".bright_red().bold())
            }
        };
    }
    pub fn call(url:&str) {
        blocking::get(url).safe_mas("call", "the server or website is up", None);
    }
}

pub mod standard {
    use std::{ env::*, io::stdin, path::PathBuf};
    use colored::Colorize;

    use crate::backend::safe::SafeT;

    pub fn input() -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).safe("Parsing Error");
        let input = input.trim().to_string();

        input
    }

    pub fn tell() -> PathBuf {
        let path = match current_dir() {
            Ok(o) => o,
            Err(e) => {
                eprintln!("([Error]~>{}: due to {}" , "Error".red() , e);
                PathBuf::new() }};
        path
    }
}

pub mod tokenization {
    use colored::Colorize;

    pub fn proc(input:String) -> Vec<String> {
        let split = match shlex::split(&input) {
            Some(o) => o,
            None => {
                eprintln!("([Error]~>{}: due to Tokenizer is not working!!" , "Error".red());
                return Vec::new();
            }
        };
        split
    }
    pub fn token(data:&[String] , index:usize ) -> String {
        let token = match data.get(index).map(|s| s.as_str()) {
            Some(t) => t,
            None => {
                return "".to_string();
            }
        };
        token.to_string()
    }
}

pub mod safe {
    use std::{fs::{File, Metadata}, io::ErrorKind, ops::Add, path::PathBuf, process::Output};

    use colored::Colorize;
    use reqwest::{StatusCode};
    use crate::backend::{standard::tell};

    pub trait Safe {
        type Out;

        fn safe(self , err_res:Option<&str>);
        fn safe_mas(self , mas1:&str , mas1:&str , err_res:Option<&str>);
        fn safe_w_res(self , err_res:Option<&str>) -> Self; 
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:Option<&str>) -> Self;
    }

    pub trait SafeT<T> {
        fn safe(self , err_res:&str);
        fn safe_mas(self , mas1:&str , mas1:&str , err_res:&str);
    }

    pub fn errmes(e: Option<&std::io::Error> , e2:Option<&reqwest::Error> , err_res:Option<&str>) {
        let path = tell();

        let e = e;
        if let Some(e) = e {
            if let Some(err_res) = err_res {
                match e.kind() {
                    ErrorKind::NotFound => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the requested resource was not found".bright_red().bold() , err_res.bright_yellow().bold());
                    }
                    ErrorKind::IsADirectory => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "is a directory".bright_red().bold() , err_res.bright_yellow().bold());
                    }
                    ErrorKind::Interrupted => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "operation interrupted".bright_red().bold() , err_res.bright_yellow());
                    }
                    ErrorKind::InvalidInput => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "invalid input".bright_red().bold() , err_res.bright_yellow());
                    }
                    ErrorKind::DirectoryNotEmpty => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "directory not empty".bright_red().bold() , err_res.bright_yellow());
                    }
                    ErrorKind::InvalidFilename => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Invalid file name".bright_red().bold() , err_res.bright_yellow());
                    }
                    ErrorKind::FileTooLarge => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "The file is to large".bright_red().bold() , err_res.bright_yellow());
                    }
                    ErrorKind::NotADirectory => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "is not a directory".bright_red().bold() , err_res.bright_yellow());
                    }
                    ErrorKind::PermissionDenied => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Permission denied".bright_red().bold() , err_res.bright_yellow());
                    }
                    ErrorKind::ReadOnlyFilesystem => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Read only file".bright_red().bold() , err_res.bright_yellow());
                    }
                    ErrorKind::InvalidData => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Invalid data".bright_red().bold() , err_res.bright_yellow());
                    }
                    ErrorKind::StorageFull => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Storage is full try to free up some storage and try again".bright_red().bold() , err_res.bright_yellow());
                    }
                    ErrorKind::Unsupported => {
                        println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "Unsupported operation".bright_red().bold() , err_res.bright_yellow());
                    }
                    _ => {
                        eprintln!("[{path:?}]~>{}: due to [ \x1b[31m{e}\x1b[0m ]" , "Error".bright_red().bold())
                    }
                }
            }
        }

        let e2 = e2;
        if let Some(e2) = e2 {
            if let Some(e2) = e2.status() {
                    match e2 {
                        StatusCode::BAD_GATEWAY => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "received an invalid response from the upstream server.".bright_red().bold() , "502".bright_yellow().bold());
                        }
                        StatusCode::BAD_REQUEST => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the request is invalid or malformed.".bright_red().bold() , "400".bright_yellow().bold());
                        }
                        StatusCode::CONFLICT => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the request could not be completed due to a resource state conflict".bright_red().bold() , "409".bright_yellow().bold());
                        }
                        StatusCode::FAILED_DEPENDENCY => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the request failed because a previous request it depends on failed.".bright_red().bold() , "424".bright_yellow().bold());
                        }
                        StatusCode::FORBIDDEN => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "you do not have permission to access this resource.".bright_red().bold() , "403".bright_yellow().bold());
                        }
                        StatusCode::GATEWAY_TIMEOUT => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the server did not receive a timely response from the upstream server.".bright_red().bold() , "504".bright_yellow().bold());
                        }
                        StatusCode::GONE => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the requested resource is no longer available and will not be available again.".bright_red().bold() , "410".bright_yellow().bold());
                        }
                        StatusCode::HTTP_VERSION_NOT_SUPPORTED => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the server does not support the HTTP protocol version used in the request".bright_red().bold() , "505".bright_yellow().bold());
                        }
                        StatusCode::INTERNAL_SERVER_ERROR => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the server encountered an unexpected condition that prevented it from fulfilling the request".bright_red().bold() , "500".bright_yellow().bold());
                        }
                        StatusCode::METHOD_NOT_ALLOWED => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the HTTP method used is not supported for this resource.".bright_red().bold() , "405".bright_yellow().bold());
                        }
                        StatusCode::MISDIRECTED_REQUEST => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the request was directed to a server that is not able to produce a valid response.".bright_red().bold() , "421".bright_yellow().bold());
                        }
                        StatusCode::NETWORK_AUTHENTICATION_REQUIRED => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the client needs to authenticate to gain network access.".bright_red().bold() , "511".bright_yellow().bold());
                        }
                        StatusCode::NON_AUTHORITATIVE_INFORMATION => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the returned metadata is not from the original server but from a local or third-party copy".bright_red().bold() , "203".bright_yellow().bold());
                        }
                        StatusCode::NOT_ACCEPTABLE => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the requested resource cannot generate a response acceptable to the client.".bright_red().bold() , "406".bright_yellow().bold());
                        }
                        StatusCode::NOT_EXTENDED => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the request requires further extensions which the server did not receive.".bright_red().bold() , "510".bright_yellow().bold());
                        }
                        StatusCode::NOT_FOUND => {
                            println!("[{path:?}]~>{}: due to [{}: <{}> ]" , "Error".bright_red().bold() , "the requested resource could not be found on the server".bright_red().bold() , "404".bright_yellow().bold());
                        }
                        _ => {}
                }
            }
        }
    }   


    impl Safe for std::io::Result<File> {
        type Out = File;

        fn safe(self , err_res:Option<&str>) {
            match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:Option<&str>) {
            let path = tell();

            match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return;
                }
            }
        }
        fn safe_w_res(self , err_res:Option<&str>) -> Self {
            match self {
                Ok(o) => {
                    return Ok(o);
                },

                Err(e) => {
                    errmes(Some(&e), None, err_res);
                    return Err(e);
                }
            }
        }
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:Option<&str>) -> Self {
            let path = tell();

            let s = match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    o
                }
                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return Err(e);
                }
            };
            return Ok(s);
        }
    }
    
    impl<T: Add<Output = T> + Copy > SafeT<T> for std::io::Result<T> {
        fn safe(self , err_res:&str) {
            match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(Some(&e), None,Some(err_res));
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:&str )  {
            let path = tell();

            match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(Some(&e),None, Some(err_res));
                    return;
                }
            }
        }
    }

    impl Safe for std::io::Result<PathBuf> {
        type Out = PathBuf;

        fn safe(self , err_res:Option<&str>)  {
             match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:Option<&str>) {
            let path = tell();

            match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(Some(&e), None, err_res);
                    return;
                }
            }
        }
        fn safe_w_res(self , err_res:Option<&str>) -> Self {
            match self {
                Ok(o) => {
                    return Ok(o);
                },

                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return Err(e);
                }
            }
        }
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:Option<&str>) -> Self {
            let path = tell();

            let s = match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    o
                }
                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return Err(e);
                }
            };
            return Ok(s);
        }
    }

    impl Safe for std::io::Result<()> {
        type Out = ();

        fn safe(self , err_res:Option<&str>) {
            match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(Some(&e),None ,err_res);
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:Option<&str>) {
            let path = tell();

            match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return;
                }
            }
        }
        fn safe_w_res(self , err_res:Option<&str>) -> Self {
            match self {
                Ok(o) => {
                    return Ok(o);
                },

                Err(e) => {
                    errmes(Some(&e),None,  err_res);
                    return Err(e);
                }
            }
        }
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:Option<&str>) -> Self {
            let path = tell();

            let s = match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    o
                }
                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return Err(e);
                }
            };
            return Ok(s);
        }
    }
    
    impl Safe for std::io::Result<Metadata> {
        type Out = Metadata;

        fn safe(self , err_res:Option<&str> ) {
            match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:Option<&str>) {
            let path = tell();

             match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(Some(&e), None, err_res);
                    return;
                }
            }
        }
        fn safe_w_res(self , err_res:Option<&str>) -> Self {
            match self {
                Ok(o) => {
                    return Ok(o);
                },

                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return Err(e);
                }
            }
        }
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:Option<&str>) -> Self {
            let path = tell();

            let s = match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    o
                }
                Err(e) => {
                    errmes(Some(&e), None,err_res);
                    return Err(e);
                }
            };
            return Ok(s);
        }
    }

    impl Safe for std::io::Result<Output> {
        type Out = Output;

        fn safe(self , err_res:Option<&str> ) {
            match self {
                Ok(_) => return,

                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return;
                }
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:Option<&str>) {
            let path = tell();

             match self {
                Ok(_) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return;
                }
                Err(e) => {
                    errmes(Some(&e), None, err_res);
                    return;
                }
            }
        }
        fn safe_w_res(self , err_res:Option<&str>) -> Self {
            match self {
                Ok(o) => {
                    return Ok(o);
                },

                Err(e) => {
                    errmes(Some(&e),None ,err_res);
                    return Err(e);
                }
            }
        }
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:Option<&str>) -> Self {
            let path = tell();

            let s = match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    o
                }
                Err(e) => {
                    errmes(Some(&e),None, err_res);
                    return Err(e);
                }
            };
            return Ok(s);
        }
    }
    
    impl Safe for std::result::Result<reqwest::blocking::Response , reqwest::Error> {
        type Out = reqwest::blocking::Response;

        fn safe(self , err_res:Option<&str>) {
            match self {
                Ok(_) => return,
                Err(e) => {
                    errmes(None,Some(&e), err_res);
                    return;
                }  
            }
        }
        fn safe_mas(self , mas1:&str , mas2:&str , err_res:Option<&str>) {
            let path = tell();

            match self {
                Ok(o) => {
                    if o.status().is_success() {
                        println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    }
                    else {
                        println!("[{path:?}]~>{}: HTTP Status Code [{}]", "Error".bright_red().bold(), o.status().as_u16().to_string().bright_yellow().bold());
                    }
                    return;
                }
                Err(e) => {
                    errmes(None,Some(&e), err_res);
                    return;
                }
            }
        }
        fn _safe_mas_w_res(self , mas1:&str , mas2:&str , err_res:Option<&str>) -> Self {
            let path = tell();

            match self {
                Ok(o) => {
                    println!("[{path:?}]~>{}: [{}]" , mas1.bright_green().bold() , mas2.bright_green().bold());
                    return Ok(o);
                }
                Err(e) => {
                    errmes(None, Some(&e), err_res);
                    return Err(e);
                }
            }
        }
        fn safe_w_res(self , err_res:Option<&str>) -> Self {
            match self {
                Ok(o) => return Ok(o),
                Err(e) => {
                    errmes(None,Some(&e), err_res);
                    return Err(e);
                }
            }
        }
    }
}

pub mod clean {
    use std::{fs::{File }, io::Read};
    use crate::{backend::safe::{Safe, SafeT}};
    
    pub fn read_file_cont(path:&str) -> std::io::Result<String> {
        let mut txtf = File::open(&path).safe_w_res(Some(&path))?;
        let mut readed = String::new();
        txtf.read_to_string(&mut readed).safe(&path);
        return Ok(readed);
    }
}
