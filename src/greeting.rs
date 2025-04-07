use super::terminal_utils::print_center;
use console::{Key, Term, style};

/// Creates the greeting
pub fn greet() {
    let ship = "                                             ..:..                              
                                               :.                               
                                               :.                               
                                            +  :. =++.                          
                                            +. :. =++.                          
                                           .+..:..=++..                         
                        ..   ...         ..:::::-:----:-...                     
                    ........::::-      .+++++++++++++++++++.                    
     . ::::--      .::::::::::::::::::::::::::::::::::::::::::::   ......::     
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=======
 +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
 .#############################################################################.
 .###########################################################################*. 
 ..........................................................................";
    let battleship = r"   ___       __  __  __    ______   _    
  / _ )___ _/ /_/ /_/ /__ / __/ /  (_)__ 
 / _  / _ `/ __/ __/ / -_)\ \/ _ \/ / _ \
/____/\_,_/\__/\__/_/\__/___/_//_/_/ .__/
                                  /_/    ";
    let term = Term::buffered_stdout();
    term.set_title("Battle Ship");

    print_center(&term, &style(ship).blue().to_string());
    print_center(&term, &style(battleship).bold().to_string());
    print_center(
        &term,
        &format!(
            "\n- Press {} or {} to Start -",
            style("Space").blue().bold(),
            style("Enter").blue().bold()
        ),
    );
    term.flush();

    // Start when the Space or Enter key are pressed
    loop {
        let key = term.read_key().expect("Key unable to be read.");
        match key {
            Key::Char(' ') => break,
            Key::Enter => break,
            _ => (),
        }
    }
    term.clear_last_lines(1);
    term.flush();
}
