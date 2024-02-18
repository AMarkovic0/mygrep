    use std::env;
    use std::process::Command;
    use std::io::BufReader;
    use std::io::stdin;
    use std::io::Read;
    use std::io::BufRead;

    fn check_for(arg: &str) -> bool {
        let mut ret = false;

        if let Some(_help) = env::args().find(|x| x == arg) {
            ret = true;
        } else if env::args().len() == 1 {
            ret = true;
        }

        ret
    }

    pub fn check_for_help() -> bool {
        check_for("--help")
    }

    pub fn check_for_history() -> bool {
        check_for("--history")
    }

    pub fn print_help() {
        println!(
            "Opens seleced file, on selected line from grep recursive search (grep -rn ...).\n"
        );
        println!(
            "Usage: pgrep [OPTION]... PATTERN [FILE]...
Search for PATTERN in each FILE and opens file on selected location.
Example: pgrep -i --include=*.c 'hello world' main.c

--history Prints history of pgrap call arguments
--help Prints help message
"
        );
        println!("Here is how the grep commands works: \n");
        Command::new("grep")
            .arg("--help")
            .spawn()
            .expect("ERROR: Failed to spwan grep")
            .wait()
            .expect("ERROR: Grep failed to execute");
    }

    pub fn select_output() -> Option<usize> {
        let mut usr_in = String::new();

        let _ = BufReader::new(stdin().take(1024)).read_line(&mut usr_in);

        match usr_in.trim_end().parse::<u32>() {
            Ok(res) => Some(res as usize),
            Err(_) => None
        }
    }
