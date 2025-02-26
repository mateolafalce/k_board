use k_board::{keyboard::Keyboard, keys::Keys};

fn main() {
    menu(0);
    let keyboard = Keyboard::new();
    for key in keyboard {
        match key {
            Keys::Up => menu(0),
            Keys::Down => menu(1),
            Keys::Enter => break,
            _ => {}
        }
    }
}

fn menu(operation: u8) {
    std::process::Command::new("clear").status().unwrap();
    let mut op: Vec<char> = vec!['*', ' '];
    if operation == 1 {
        op[0] = ' ';
        op[1] = '*';
    }
    println!(
        "[{}] I use k_board lightweight software\n[{}] I use heavyweight software",
        op[0], op[1]
    );
}
