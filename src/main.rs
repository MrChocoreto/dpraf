//<<----Mods----->>

mod args;
mod read;
mod write;


//<<----Creates----->>

use crate::read::read_file;
use crate::write::make_page;


//<<----MainFunc----->>

fn main() {
    read_file::read_file().expect("");
    make_page::build()
}


//<<----Funcs----->>


