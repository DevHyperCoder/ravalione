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

use std::{fs::File, path::Path};

use crate::error::RlError;

/// Equivalent to UNIX `touch` command.
/// Creates the param* files from the current working directory
pub fn touch(params: Vec<&str>) -> Result<(), RlError> {
    for param in params {
        let path = Path::new(param);

        if path.exists() {
            println!("[WARN] {} already exists! Skipping.", param);
            continue;
        }

        if let Err(e) = File::create(path) {
            return Err(RlError::RlFs(format!(
                "Unable to create {}\n{}",
                param,
                e.to_string()
            )));
        }
    }

    Ok(())
}
