    mod backend;
    mod apps;
    use crate::backend::safe::{ ErrH, HyperkitError, Ugh};
    use crate::backend::{commands, standard::tell, parser::* };
    use std::{env::* , borrow::Cow::{self, Owned}};
    use colored::*;

    use rustyline::{Completer, Hinter, Validator , error::ReadlineError, completion::FilenameCompleter , highlight::{CmdKind, Highlighter, MatchingBracketHighlighter}, hint::HistoryHinter, 
    validate::MatchingBracketValidator , Cmd , CompletionType , Config , EditMode, Editor , KeyEvent , Helper 
    };

    #[derive(Helper , Completer , Hinter , Validator)]
    pub struct Enveditor {
        #[rustyline(Completer)]
        comp:FilenameCompleter,
        hig:MatchingBracketHighlighter,
        #[rustyline(Validator)]
        val:MatchingBracketValidator,
        #[rustyline(Hinter)]
        hin:HistoryHinter,
    }

    impl Highlighter for Enveditor {
        fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
            Owned("\x1b[1m".to_owned() + hint + "\x1b[0m")
        }
        fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
            self.hig.highlight(line, pos)
        }
        fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
            self.hig.highlight_char(line, pos, kind)
        }
    }

    const GITHUBLINK:&str = "https://github.com/mohamemd-v1/Shell-like-toolbox-.git";

    fn main() -> std::result::Result<() , HyperkitError> {
        println!("*{} {} {} to see all the commands , {} to list all the available built in apps , {} for about" , "Enter".green()  , "help".red() ,"--commands".bright_purple() , "--built-in-apps".bright_purple() , "--about".bright_purple() );
        let home = match home_dir() {
            Some(h) => h,
            None => {
                println!(">{}: home dir does not exsit?" , "Error".red());
                return Ok(());
            }
        };

        let _ = set_current_dir(home).errh(None);

        let configdef = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Emacs)
        .build();

        let enveditor = Enveditor {
            hig:MatchingBracketHighlighter::new(),
            comp:FilenameCompleter::new(),
            val:MatchingBracketValidator::new(),
            hin:HistoryHinter::new(),
        };


        let mut def = Editor::with_config(configdef).unwrap();
        def.set_helper(Some(enveditor));
        def.bind_sequence(KeyEvent::alt('f'), Cmd::ForwardSearchHistory);
        def.bind_sequence(KeyEvent::alt('b'), Cmd::HistorySearchBackward);


        def.load_history("/home/mohammed/programming/Rust/practice/HyperKit/hyper/hyperhis.txt").unwrap_or_else(|e| {
            let path = tell();
            match e {
                ReadlineError::Io(e) => {
                    match e.kind() {
                        std::io::ErrorKind::NotFound => eprintln!("[{path:?}]~>{} due to {}" , "Error".bold().bright_red() , "History file doesn't exist".bright_red().bold()),
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    loop {
        let path = tell();
        
        let f  = format!("[{path:?}]~>");
        
        let void = match def.readline(&f) {
            Ok(o) => {
                def.add_history_entry(o.as_str()).unwrap();
                o
            },

            Err(e) => match e {
                ReadlineError::Interrupted => {
                    break;
                }
                _ => {
                    eprintln!("[{path:?}]~>{}: due to {e}" , "Error".red());
                    return Ok(());
                }
            }
        };

        let data = parser(&void);
        let tok1 = match data.get(0) {
            Some(t) => t.to_owned(),
                None => {
                    continue;
            }
        };

        match tok1.trim() {
            "help" => {
                let tok1 = token(&data, 1)?;
                commands::help(tok1);
            }
            "clean" => {
                commands::clean().unwrap_or_default();
            }
            "go" => {
                let tok2  = token(&data, 1)?;
                if &tok2 == "back" {
                    commands::go("..").unwrap_or_default();
                }
                else {
                    let tok2 = token(&data, 1)?;
                    commands::go(&tok2).unwrap_or_default();
                }
            }
            "wh" => {
                commands::wh().unwrap_or_default();
            }
            "see" => {
                commands::see().unwrap_or_default();
            }
            "peek" => {
                let tok2 = token(&data, 1)?;
                commands::peek(&tok2).unwrap_or_default();
            }
            "mk" => {
                let tok2 = token(&data, 1)?;
                commands::mk(&tok2).unwrap_or_default();
            }
            "burn" => {
                let tok2 = token(&data, 1)?;
                commands::burn(&tok2).unwrap_or_default();
            }
            "rn" => {
                let tok1 = token(&data, 1)?;
                let tok2 = token(&data, 2)?;
                commands::rn(&tok1 , &tok2).unwrap_or_default();
            }
            "clone" => {
                let tok1 = token(&data, 1)?;
                let tok2 = token(&data, 2)?;
                commands::clone(tok1 , tok2).unwrap_or_default();
            }
            "forge" => {
                let tok1 = token(&data, 1)?;
                commands::forge(tok1).unwrap_or_default();
            }
            "run" => {
                let tok1 = token(&data , 1)?;
                commands::run(&tok1).unwrap_or_default();
            }
            "calc" => {
                let tok1 = token(&data , 1)?;
                apps::calc(tok1);
            }
            "time" => {
                apps::time();
            }
            "mv" => {
                let tok1 = token(&data, 1)?;
                let tok2 = token(&data, 2)?;
                commands::mv(&tok1, &tok2).unwrap_or_default();
            }
            "ship" => {
                let ttype = token(&data, 1)?;
                let flag  = token(&data, 2)?;
                let fname = token(&data, 3)?;
                let outname = token(&data, 4)?;
                apps::ship(ttype, flag , fname , outname).unwrap_or_default();
            }
            "transmute" => {
                let ttype = token(&data, 1)?;
                let flag  = token(&data, 2)?;
                let fname = token(&data, 3)?;
                let outname = token(&data, 4)?;
                apps::transmute(&ttype, &flag, &fname, &outname).unwrap_or_default();
            }
            "find" => {
                let fpath = token(&data, 1)?;
                commands::find(&fpath).ugh();
            }
            "ps" => {
                let tok1 = token(&data, 1)?;
                let tok2 = token(&data, 2)?;
                let tok2 = tok2.parse().map(|e: usize| e as usize).unwrap_or_default();
                
                commands::ps(&tok1 , tok2).ugh();
            }
            "stop" => {
                let tok1 = token(&data, 1)?.parse().map(|e:i32| e as i32).errh(Some(tok1.to_string())).unwrap_or_default();
                commands::stop(tok1);    
            }
            "end" => {
                break;
            }
            _ => {
                if !tok1.is_empty() {
                    println!("[{path:?}]~>{}: [\x1b[36m{}\x1b[0m] please try again" , "unknown command".bright_red() ,&tok1 );
                    continue;
                }
            }
        }
    }
    let path = tell();
        def.save_history("/home/mohammed/programming/Rust/practice/HyperKit/hyper/hyperhis.txt").unwrap_or_else(|e| 
            {eprintln!("[{path:?}]~>{}: due to {}" , "Error".bright_red().bold() , e.to_string().bright_red().bold())});
        
        Ok(())
    }
