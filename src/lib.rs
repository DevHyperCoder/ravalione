/*
 *    ravalione - easy project templates
 *    Copyright (C) 2021 DevHyperCoder
 *
 *    This program is free software: you can redistribute it and/or modify
 *    it under the terms of the GNU General Public License as published by
 *    the Free Software Foundation, either version 3 of the License, or
 *    (at your option) any later version.
 *
 *    This program is distributed in the hope that it will be useful,
 *    but WITHOUT ANY WARRANTY; without even the implied warranty of
 *    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *    GNU General Public License for more details.
 *
 *    You should have received a copy of the GNU General Public License
 *    along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 *    Contact the author: <devan at devhypercoder dot com>
 */

//! ravalione - easy project templates

#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

#[macro_use]
extern crate pest_derive;

/// Instruction file parser
#[allow(missing_docs)]
pub mod parser;

/// File operations
pub mod file;

/// Error enums for Result
pub mod error;

/// Instruction handler and executor
pub mod instruction;

/// Command functions ie echo, cp etc
pub mod cmd;

use error::RlError;
use file::read_ravalione_instructions;

use crate::instruction::RlInstruction;

/// Main Executor
pub fn run() -> Result<(), RlError> {
    println!("ravalione");

    let file = match read_ravalione_instructions(None) {
        Ok(file) => file,
        Err(why) => return Err(why),
    };

     match RlInstruction::parse_rl_instructions(&file) {
         Ok(instructions) => {
             for instr in instructions {
                 if let Err(e) = instr.execute(){
                     return Err(e)
                 }
             }
         },
         Err(why) => return Err(why)
     }

    Ok(())
}
