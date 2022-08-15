use logos::Logos;

// ident regex: [a-zA-Z_][a-zA-Z_0-9]*

#[derive(Logos, Debug, PartialEq)]
pub(crate) enum Token {
    #[regex(r"\s@([a-zA-Z_][a-zA-Z_0-9]*)?:?([a-zA-Z_][a-zA-Z_0-9]*)?")]
    // for handle @-ident: \s@(([a-zA-Z_][a-zA-Z_0-9]s*)?:?([a-zA-Z_][a-zA-Z_0-9]*)?|-([a-zA-Z_][a-zA-Z_0-9]*)?)
    Mention,

    #[regex(r"\s#([a-zA-Z_][a-zA-Z_0-9]*)")]
    Link,

    #[regex(r"\s:([a-zA-Z_][a-zA-Z_0-9]*)(\([^\)]*\))?")]
    Command,

    #[regex(r"\{[^\}]+\}")]
    InlineBlock,

    #[regex(r"//[^\n]*\n")]
    SinglelineComment, // TODO: multiline comments

    #[error]
    Text,
}

#[derive(Debug)]
pub(crate) enum SemanticToken {
    Mention(MentionToken),
    Text(Vec<String>),
    Link(String),
    Command(String, String),
    InlineBlock(String),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub(crate) enum MentionToken {
    Narrator,
    Name(String),
    State(String),
    NameState(String, String),
}
