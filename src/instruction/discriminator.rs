#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Modifier {
    Lock,
    BranchHintTrue,
    BranchHintFalse,
    RemapVector { }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Add,
    Subtract
}