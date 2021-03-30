use crate::*;
use bogobble::*;

pub enum Item {
    Hits(usize),
    Recent(usize),
    Path(String),
    Cmd(String),
}

parser! {(File->Vec<Item>)
    (star(("\n ,".istar(),PItem).last()),EOI).first()
}

parser! {(PItem->Item)
    or!(
        ("h",common::UInt).map(|(_,n)|Item::Hits(n)),
        ("r",common::UInt).map(|(_,n)|Item::Recent(n)),
        ("p",Quoted).map(|(_,s)|Item::Path(s)),
        ("c",Quoted).map(|(_,s)|Item::Cmd(s)),
    )
}

parser! {(Quoted->String)
    ('\"',chars_until(or!(
                ('\\','\n').asv('\n'),
                ('\\','"').asv('"'),
                Any.one(),
    ),'"')).map(|(_,(b,_))|b)
}

pub fn parse_onto(h: &mut HistoryStore, s: &str) -> anyhow::Result<()> {
    Ok(())
}
