#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputParseState {
    Definitions = 0,
    Rules,
    UserSubroutines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefinitionItemType {
    Unknown = 0,
    CodeBlock,
    CodeLine,
    YytextType,
    StartCondition,
    ExclusiveCondition,
    TableSize,
    Substitution,
}

impl Default for DefinitionItemType {
    fn default() -> Self {
        DefinitionItemType::Unknown
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleItemType {
    Unknown = 0,
    CodeBlock,
    CodeLine,
    Rule,
}

impl Default for RuleItemType {
    fn default() -> Self {
        RuleItemType::Unknown
    }
}


#[derive(Debug, Default, Clone)]
pub struct DefinitionItem {
    pub item_type: DefinitionItemType,
    pub start_line: u64,
    pub end_line: u64,
    pub re_end_pos: u64,
}

#[derive(Debug, Default)]
pub struct SectionDefinitions {
    pub items: Vec<DefinitionItem>,
}

#[derive(Debug, Default, Clone)]
pub struct RuleItem {
    pub item_type: RuleItemType,
    pub start_line: u64,
    pub end_line: u64,
    pub re_end_pos: u64,
}

#[derive(Debug, Default)]
pub struct SectionRules {
    pub items: Vec<RuleItem>,
    pub rules_emerged: bool,
}

#[derive(Debug, Default)]
pub struct SectionUserSubroutines {
    pub start_line: u64,
    pub end_line: u64,
}

#[derive(Debug, Default)]
pub struct ParsedInput {
    pub lines: Vec<String>,

    pub definitions: SectionDefinitions,
    pub rules: SectionRules,
    pub user_subroutines: SectionUserSubroutines,
}