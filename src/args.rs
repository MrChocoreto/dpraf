use clap::{command, Arg, Command};


pub fn _arg_reader(){
    let match_result = command!()
    .arg(
        
        Arg::new("hola").short('o').long("hola")
    )
    .subcommand(
        Command::new("Yeppe")
            .arg(
                Arg::new("womp")
                .short('w')
                .long("womp")
            )
    )
    .get_matches();
    let data_args = match_result.subcommand_matches("Yeppe");
    if data_args.unwrap().contains_id("womp"){
        println!("Todo Bien");
    }
    else {
        println!("Mamaste");
    }

}