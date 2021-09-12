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

use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::error::RlError;

/// Read ravalione instruction file
pub fn read_ravalione_instructions(path: Option<PathBuf>) -> Result<String, RlError> {
    let file_path = match get_rl_instructions_file_path(path) {
        Err(why) => return Err(why),
        Ok(fp) => fp,
    };

    match read_to_string(&file_path) {
        Ok(file) => Ok(file),
        Err(_) => {
            let err_msg = format!("Failed to read instruction file at {:?}", file_path);
            Err(RlError::RlInstructionFile(err_msg))
        }
    }
}

/// Provided path takes importance over the others. If not provided, its taken from [".ravalione","ravalione.rc",".rli",".rlrc"]
pub fn get_rl_instructions_file_path(path: Option<PathBuf>) -> Result<PathBuf, RlError> {
    // Supported file list
    const FILE_LIST: [&str; 4] = [".ravalione", "ravalione.rc", ".rli", ".rlrc"];

    if let Some(path) = path {
        Ok(path)
    } else {
        let mut final_path = None;
        for file in FILE_LIST {
            if Path::new(file).exists() {
                final_path = Some(file);
                break;
            }
        }

        match final_path {
            None => Err(RlError::RlInstructionFile(
                "Failed to find instruction file.
Could not find .ravalione ravalione.rc .rli .rlrc"
                    .to_string(),
            )),
            Some(path) => Ok(PathBuf::from(path)),
        }
    }
}
