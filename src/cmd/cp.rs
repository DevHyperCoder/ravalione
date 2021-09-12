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
    fs::{self, copy},
    path::{Path, PathBuf},
};

use crate::error::RlError;

/// Copies SOURCE (param[0]) to DEST (param[1])
/// Errors if number of arguments is less than or more than 2
/// FS errors are returned as RlError::RlFs
pub fn cp(param: Vec<&str>) -> Result<(), RlError> {
    if param.len() < 2 {
        return Err(RlError::RlInsuffcientArgs(format!(
            "CP provided with {} argument(s). Please provide SOURCE and DEST.",
            param.len()
        )));
    }

    if param.len() > 2 {
        return Err(RlError::RlInsuffcientArgs(format!(
            "CP provided with {} arguments. Please provide SOURCE and DEST.",
            param.len()
        )));
    }

    if let Err(why) = copy_file_or_dir(PathBuf::from(param[0]), PathBuf::from(param[1])) {
        return Err(RlError::RlFs(format!(
            "Could not copy {} to {}\n{}",
            param[0],
            param[1],
            why.to_string()
        )));
    }

    Ok(())
}

fn copy_file_or_dir(from: PathBuf, to: PathBuf) -> Result<(), std::io::Error> {
    if from.is_file() {
        if let Err(why) = copy(from, to) {
            return Err(why);
        }
        Ok(())
    } else {
        copy_dir(from, to)
    }
}

fn copy_dir<U: AsRef<Path>, V: AsRef<Path>>(from: U, to: V) -> Result<(), std::io::Error> {
    let mut stack = vec![PathBuf::from(from.as_ref())];

    let output_root = PathBuf::from(to.as_ref());
    let input_root = PathBuf::from(from.as_ref()).components().count();

    while let Some(working_path) = stack.pop() {
        // Generate a relative path
        let src: PathBuf = working_path.components().skip(input_root).collect();

        // Create a destination if missing
        let dest = if src.components().count() == 0 {
            output_root.clone()
        } else {
            output_root.join(&src)
        };
        if fs::metadata(&dest).is_err() {
            fs::create_dir_all(&dest)?;
        }

        for entry in fs::read_dir(working_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                match path.file_name() {
                    Some(filename) => {
                        let dest_path = dest.join(filename);
                        fs::copy(&path, &dest_path)?;
                    }
                    None => {
                        println!("failed: {:?}", path);
                    }
                }
            }
        }
    }

    Ok(())
}
