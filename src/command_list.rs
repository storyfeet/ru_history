use crate::CommandData;

pub struct CommandRes<'a> {
    cmd: &'a str,
    dat: &'a CommandData,
    score: u64,
}

pub struct CommandList<'a> {
    v: Vec<CommandRes<'a>>,
}

impl<'a> CommandList<'a> {
    pub fn sort_for_cmd(&mut self) {}
}

pub fn build_command_list<'a, I: Iterator<Item = (&'a String, &'a CommandData)>>(
    mut it: I,
    dir: &str,
) -> CommandList<'a> {
    CommandList {
        v: it
            .map(|(cmd, dat)| {
                let mut score = dat.hits;
                if let Some(v) = dat.paths.get(dir) {
                    score += v.hits;
                }

                CommandRes { cmd, dat, score }
            })
            .collect(),
    }
}
