/***************************************************************************************
 *   termio.rs  --  This file is part of k_board.                                      *
 *                                                                                     *
 *   Copyright (C) 2023 Mateo Lafalce                                                  *
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

use std::io::Write;

#[allow(non_camel_case_types)]
type tcflag_t = std::ffi::c_uint;

const TCSANOW: std::ffi::c_int = 0;
const TCSADRAIN: std::ffi::c_int = 0;
const ICANON: tcflag_t = 0x00000002;
const ECHO: tcflag_t = 0x00000008;
const IXON: tcflag_t = 0x00000400;
const IXANY: tcflag_t = 0x00000800;

pub fn setup_raw_mode() -> std::io::Result<termios> {
    let mut termios: termios = unsafe { std::mem::zeroed() };
    if unsafe { tcgetattr(TCSANOW, &mut termios) } < 0 {
        return Err(std::io::Error::last_os_error());
    }
    let original_termios = termios.clone();
    termios.c_lflag &= !(ICANON | ECHO);
    termios.c_iflag &= !(IXON | 0x00001000 | IXANY);
    termios.c_cc[6] = 0;
    termios.c_cc[5] = 1;
    if unsafe { tcsetattr(TCSANOW, TCSADRAIN, &termios) } < 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(original_termios)
}

pub fn restore_termios(original_termios: &termios) -> std::io::Result<()> {
    if unsafe { tcsetattr(TCSANOW, TCSADRAIN, original_termios) } < 0 {
        return Err(std::io::Error::last_os_error());
    }
    Ok(())
}

pub fn restore(termios_enviroment: &termios) {
    std::io::stdout().flush().unwrap();
    restore_termios(termios_enviroment).unwrap();
}

/// look more at https://linux.die.net/man/3/termios
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
    /// tcsetattr() sets the parameters associated with the terminal
    /// (unless support is required from the underlying hardware that is
    /// not available) from the termios structure referred to by termios_p.
    /// optional_actions specifies when the changes take effect
    pub fn tcsetattr(
        fd: std::ffi::c_int,
        optional_actions: std::ffi::c_int,
        termios_p: *const termios,
    ) -> std::ffi::c_int;
    /// tcgetattr() gets the parameters associated with the object referred
    /// by fd and stores them in the termios structure referenced by termios_p.
    /// This function may be invoked from a background process; however, the terminal
    /// attributes may be subsequently changed by a foreground process.
    pub fn tcgetattr(fd: std::ffi::c_int, termios: *mut termios) -> std::ffi::c_int;
}
