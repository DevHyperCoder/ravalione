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

use std::fs::remove_file;

use crate::error::RlError;

/// Removes param*
/// FS errors are returned as RlError::RlFs
pub fn rm(params: Vec<&str>) -> Result<(), RlError> {
    for param in params {
        if let Err(why) = remove_file(param) {
            return Err(RlError::RlFs(format!(
                "Could not remove {}\n{}",
                param,
                why.to_string()
            )));
        }
    }

    Ok(())
}
