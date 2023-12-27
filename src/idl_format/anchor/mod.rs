use serde::Deserialize;

use super::{IdlCodegenModule, IdlFormat};

use self::{
    accounts::{AccountsCodegenModule, NamedAccount},
    errors::{ErrorEnumVariant, ErrorsCodegenModule},
    events::{Event, EventsCodegenModule},
    instructions::{IxCodegenModule, NamedInstruction},
    typedefs::{NamedType, TypedefsCodegenModule},
};

pub mod accounts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod typedefs;

#[derive(Deserialize)]
pub struct AnchorIdl {
    pub name: String,
    pub version: String,
    pub metadata: Option<Metadata>,
    pub accounts: Option<Vec<NamedAccount>>,
    pub types: Option<Vec<NamedType>>,
    pub instructions: Option<Vec<NamedInstruction>>,
    pub errors: Option<Vec<ErrorEnumVariant>>,
    pub events: Option<Vec<Event>>,
}

#[derive(Deserialize)]
pub struct Metadata {
    pub address: String,
}

impl IdlFormat for AnchorIdl {
    fn program_name(&self) -> &str {
        &self.name
    }

    fn program_version(&self) -> &str {
        &self.version
    }

    fn program_address(&self) -> Option<&str> {
        self.metadata.as_ref().map(|m| m.address.as_ref())
    }

    /// Anchor IDLs dont seem to have an identifier,
    /// assume unindentified IDLs are anchor by default.
    /// -> Make sure to try deserializing Anchor last
    fn is_correct_idl_format(&self) -> bool {
        true
    }

    fn has_errors(&self) -> bool {
        self.errors.is_some()
    }

    fn modules<'me>(&'me self, args: &'me crate::Args) -> Vec<Box<dyn IdlCodegenModule + 'me>> {
        let mut res: Vec<Box<dyn IdlCodegenModule + 'me>> = Vec::new();
        if let Some(v) = &self.accounts {
            res.push(Box::new(AccountsCodegenModule {
                cli_args: args,
                named_accounts: v,
            }));
        }
        if let Some(v) = &self.r#types {
            res.push(Box::new(TypedefsCodegenModule {
                cli_args: args,
                named_types: v,
            }));
        }
        if let Some(v) = &self.instructions {
            res.push(Box::new(IxCodegenModule {
                program_name: self.program_name(),
                instructions: v,
            }));
        }
        if let Some(v) = &self.errors {
            res.push(Box::new(ErrorsCodegenModule {
                program_name: self.program_name(),
                variants: v,
            }));
        }
        if let Some(v) = &self.events {
            res.push(Box::new(EventsCodegenModule(v)));
        }
        res
    }
}
