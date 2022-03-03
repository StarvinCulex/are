const CMD_SPLITTER: [&str; 1] = [";"];

impl Player {
    #[inline]
    fn parse_command<'b, 'c, CMDArgs: Iterator<Item = &'c str>>(
        cmd: &'b str,
        mut args: CMDArgs,
    ) -> Result<Command, ()> {
        Ok(match cmd {
            // set element: `sete <value> <x> <y>`
            "set" => {
                let sub_op = args.next().ok_or(())?;
                match sub_op {
                    "element" => {
                        let value = args.next().ok_or(())?.parse::<u32>().or(Err(()))?;
                        let x = args.next().ok_or(())?.parse::<isize>().or(Err(()))?;
                        let y = args.next().ok_or(())?.parse::<isize>().or(Err(()))?;
                        Command::Mut(MutCommand::SetElement {
                            pos: Coord(x, y),
                            value,
                        })
                    }
                    _ => return Err(()),
                }
            }
            // light <x> <y>
            "light" => {
                let x = args.next().ok_or(())?.parse::<isize>().or(Err(()))?;
                let y = args.next().ok_or(())?.parse::<isize>().or(Err(()))?;
                Command::Const(ConstCommand::Light { pos: Coord(x, y) })
            }
            // blind
            "mode" => Command::Const(ConstCommand::Mode {
                mode: {
                    let x = args.next().ok_or(())?;
                    match x.to_lowercase().as_str() {
                        "blind" => Mode::Blind,
                        "monitor" => Mode::Monitor,
                        _ => return Err(()),
                    }
                },
            }),

            _ => return Err(()),
        })
    }
}

enum Command {
    Const(ConstCommand),
    Mut(MutCommand),
}

enum ConstCommand {
    Light { pos: Coord<isize> },
    Mode { mode: Mode },
    Get {},
}

impl ConstCommand {
    fn exec(self, player: &mut Player, cosmos: &Cosmos) {
        match self {
            ConstCommand::Light { pos } => {
                cosmos.plate[pos].body.element.light();
            }
            ConstCommand::Mode { mode } => {
                player.mode = mode;
            }
            ConstCommand::Get {} => {
                unimplemented!();
            }
        }
    }
}

enum MutCommand {
    SetElement { pos: Coord<isize>, value: u32 },
}

impl MutCommand {
    fn exec(self, player: &mut Player, cosmos: &mut Cosmos) {
        match self {
            MutCommand::SetElement { pos, value } => {
                cosmos.plate[pos].body.element.set_raw(value);
            }
        }
    }
}
