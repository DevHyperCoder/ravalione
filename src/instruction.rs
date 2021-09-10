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

use crate::{
    cmd::{echo::echo, error::error, touch::touch},
    parser::RLParser,
    parser::Rule,
    RlError,
};
use pest::Parser;

/// Instruction with params
/// Call execute() to actually run the instruction.
#[derive(Debug)]
pub enum RlInstruction<'a> {
    /// Echo Instruction. Prints param* to stdout
    Echo(Vec<&'a str>),

    /// Error Instruction. Prints param* to stderr
    Error(Vec<&'a str>),

    /// UNIX `touch` command. Creates param* files
    Touch(Vec<&'a str>),
}

impl RlInstruction<'_> {
    /// Execute necessary instruction.
    /// cmd module contains submodules for supported instructions.
    pub fn execute(self) -> Result<(), RlError> {
        match self {
            RlInstruction::Echo(params) => echo(params),
            RlInstruction::Error(params) => error(params),
            RlInstruction::Touch(params) => touch(params),
        }
    }

    fn get_rl_instruction<'a>(
        cmd: &str,
        params: Vec<&'a str>,
    ) -> Result<RlInstruction<'a>, RlError> {
        match cmd {
            "ECHO" => Ok(RlInstruction::Echo(params)),
            "ERROR" => Ok(RlInstruction::Error(params)),
            "TOUCH" => Ok(RlInstruction::Touch(params)),
            _ => Err(RlError::RlCommandNotFound(format!(
                "Could not find command {}.",
                cmd
            ))),
        }
    }

    /// Parse all RlInstruction(s) from given content.
    ///
    /// Errors:
    /// - RlError::RlInstructionParse if initial parse goes wrong.
    /// - RlError::RlCommandNotFound if given command is not found.
    pub fn parse_rl_instructions(content: &str) -> Result<Vec<RlInstruction<'_>>, RlError> {
        match RLParser::parse(Rule::main, content) {
            Ok(mut parse) => {
                let mut rl_instructions = vec![];
                let parse = parse.next().unwrap();
                for line in parse.into_inner() {
                    if line.as_rule() == Rule::cmd_statement {
                        let mut inner_rule = line.into_inner();

                        let cmd = inner_rule.next().unwrap().as_str();

                        let mut params = vec![];

                        for param in inner_rule {
                            params.push(param.as_str())
                        }

                        match RlInstruction::get_rl_instruction(cmd, params) {
                            Err(e) => return Err(e),
                            Ok(cmd) => rl_instructions.push(cmd),
                        }
                    }
                }
                Ok(rl_instructions)
            }
            Err(why) => Err(RlError::RLInstructionParse(why.to_string())),
        }
    }
}
