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

use std::fmt::Display;

#[derive(Debug)]

/// Error Types for file operations, parsing, output and more.
pub enum RlError {
    /// Rl Instruction file was not found or failed to read.
    RlInstructionFile(String),

    /// Rl Instruction Parse failed
    RLInstructionParse(String),

    /// Provided command was not found
    RlCommandNotFound(String),

    /// fs error
    RlFs(String),

    /// Insuffcient arguments provided
    RlInsuffcientArgs(String),
}

impl Display for RlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (rl_error_type, rl_error_msg) = match self {
            RlError::RlInstructionFile(e) => ("INSTRUCTION FILE", e),
            RlError::RLInstructionParse(e) => ("PARSE", e),
            RlError::RlCommandNotFound(e) => ("CMD", e),
            RlError::RlInsuffcientArgs(e) => ("ARG", e),
            RlError::RlFs(e) => ("FS", e),
        };
        write!(f, "[{}]: {}", rl_error_type, rl_error_msg)
    }
}
