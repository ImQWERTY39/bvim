use std::env;
mod editor;

fn main() {
    let file_path = env::args().last();
    editor::Editor::init(file_path);
}

/*
Similar to vim

modes
1. normal mode (esc)
view the file
    i)  in binary: displays 0s and 1s
    ii) in hex: displays 0-9 and A-F

    h or left  => moves cursor left
    j or up    => moves cursor up
    k or down  => moves cursor down
    l or right => moves cursor right

2. insert mode (i)
put value in the file
    i)  binary mode
        accepts 0s and 1s
    ii) hex mode
        acceps 0-9 and a-f (shows in uppercase)

3. select mode? should i have this? probably not


commands (:)
q => quit
w => write


other stuff to remember
i)    keeping rel file path as a variable
ii)   switching the view from binary to hex and vice versa, based on whatever it is
iii)  getting a slice of data from the file, put that on screen and load the rest



random ideas

i) line wrap(hard)?
    or limit to how many digits per line (easy)?
    or moving the document view to the right/left (impossible)?

*/
