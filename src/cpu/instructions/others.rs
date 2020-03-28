use super::Instruction;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BRK;
impl Instruction for BRK {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RTI;
impl Instruction for RTI {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RTS;
impl Instruction for RTS {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PHA;
impl Instruction for PHA {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PHP;
impl Instruction for PHP {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PLA;
impl Instruction for PLA {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PLP;
impl Instruction for PLP {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JSR;
impl Instruction for JSR {}
