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

use std::fs::rename;

use crate::error::RlError;

/// Moves SOURCE (param[0]) to DEST (param[1])
/// Errors if number of arguments is less than or more than 2
/// FS errors are returned as RlError::RlFs
pub fn mv(param: Vec<&str>) -> Result<(), RlError> {
    if param.len() < 2 {
        return Err(RlError::RlInsuffcientArgs(format!(
            "MV provided with {} argument(s). Please provide SOURCE and DEST.",
            param.len()
        )));
    }

    if param.len() > 2 {
        return Err(RlError::RlInsuffcientArgs(format!(
            "MV provided with {} arguments. Please provide SOURCE and DEST.",
            param.len()
        )));
    }

    if let Err(why) = rename(param[0], param[1]) {
        return Err(RlError::RlFs(format!(
            "Could not move {} to {}\n{}",
            param[0],
            param[1],
            why.to_string()
        )));
    }

    Ok(())
}
