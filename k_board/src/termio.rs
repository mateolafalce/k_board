/***************************************************************************************
 *   termio.rs  --  This file is part of k_board.                                      *
 *                                                                                     *
 *   Copyright (C) 2024 Mateo Lafalce                                                  *
 *                                                                                     *
 *   k_board is free software: you can redistribute it and/or modify                   *
 *   it under the terms of the GNU General Public License as published                 *
 *   by the Free Software Foundation, either version 3 of the License,                 *
 *   or (at your option) any later version.                                            *
 *                                                                                     *
 *   k_board is distributed in the hope that it will be useful,                        *
 *   but WITHOUT ANY WARRANTY; without even the implied warranty                       *
 *   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.                           *
 *   See the GNU General Public License for more details.                              *
 *                                                                                     *
 *   You should have received a copy of the GNU General Public License                 *
 *   along with this program.  If not, see http://www.gnu.org/licenses/.               *
 *                                                                                     *
 **************************************************************************************/

use std::{
    ffi::{c_int, c_uint},
    io::{stdout, Error, Result, Write},
    mem::zeroed,
};

#[cfg(any(
    feature = "ctrl_lower_letter",
    feature = "ctrl_upper_letter",
    feature = "full"
))]
use crate::keyboard::CTRL_C;

#[allow(non_camel_case_types)]
type size_t = usize;
#[allow(non_camel_case_types)]
type tcflag_t = c_uint;
#[allow(non_camel_case_types)]
type sighandler_t = size_t;

/// Interrupt from keyboard
#[cfg(any(
    feature = "ctrl_lower_letter",
    feature = "ctrl_upper_letter",
    feature = "full"
))]
pub const SIGINT: c_int = 2;

const TCSANOW: c_int = 0;
const TCSADRAIN: c_int = 0;
const ICANON: tcflag_t = 0x00000002;
const ECHO: tcflag_t = 0x00000008;
const IXON: tcflag_t = 0x00000400;
const IXANY: tcflag_t = 0x00000800;

/// Setup the raw mode in the console to take the termios struct
pub fn setup_raw_mode() -> Result<termios> {
    let mut termios: termios = unsafe { zeroed() };
    if unsafe { tcgetattr(TCSANOW, &mut termios) } < 0 {
        return Err(Error::last_os_error());
    }
    let original_termios = termios.clone();
    termios.c_lflag &= !(ICANON | ECHO);
    termios.c_iflag &= !(IXON | 0x00001000 | IXANY);
    termios.c_cc[6] = 0;
    termios.c_cc[5] = 1;
    if unsafe { tcsetattr(TCSANOW, TCSADRAIN, &termios) } < 0 {
        return Err(Error::last_os_error());
    }
    Ok(original_termios)
}

/// Restore the termios enviroment
pub fn restore(termios_enviroment: &termios) -> Result<()> {
    stdout().flush().unwrap();
    if unsafe { tcsetattr(TCSANOW, TCSADRAIN, termios_enviroment) } < 0 {
        return Err(Error::last_os_error());
    }
    Ok(())
}

/// Modify `CTRL_C` -> true, then the iterator get the change &
/// return the `Keys::Ctrl('c')` || `Keys::Ctrl('C')`
#[cfg(any(
    feature = "ctrl_lower_letter",
    feature = "ctrl_upper_letter",
    feature = "full"
))]
pub fn sig_handler() {
    let mut ctrl_c_handler = CTRL_C.lock().unwrap();
    *ctrl_c_handler = true;
}

/// look more at <https://linux.die.net/man/3/termios>
#[derive(Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: u32,
    pub c_oflag: u32,
    pub c_cflag: u32,
    pub c_lflag: u32,
    c_line: u8,
    pub c_cc: [u8; 32],
    c_ispeed: u32,
    c_ospeed: u32,
}

#[link(name = "c")]
extern "C" {
    /// `tcsetattr()` sets the parameters associated with the terminal
    /// (unless support is required from the underlying hardware that is
    /// not available) from the termios structure referred to by termios_p.
    /// optional_actions specifies when the changes take effect
    pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
    /// `tcgetattr()` gets the parameters associated with the object referred
    /// by fd and stores them in the termios structure referenced by termios_p.
    /// This function may be invoked from a background process; however, the terminal
    /// attributes may be subsequently changed by a foreground process.
    pub fn tcgetattr(fd: c_int, termios: *mut termios) -> c_int;
    /// `signal()` sets the disposition of the signal signum to handler,
    /// which is either SIG_IGN, SIG_DFL, or the address of a programmer-
    /// defined function (a "signal handler").
    pub fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;
}
