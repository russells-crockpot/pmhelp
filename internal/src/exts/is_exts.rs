use alloc::vec::Vec;
use syn::{
    parse::{Parse, ParseBuffer, Peek},
    GenericArgument, Ident, Path, PathArguments, PathSegment, Type, TypePath,
};

#[derive(PartialEq, Clone, Copy)]
enum OptionCheckerState {
    Unknown = 1,
    StdOrCore = 2,
    OptionMod = 3,
    Final = 4,
}

impl PartialEq<PathSegment> for OptionCheckerState {
    fn eq(&self, PathSegment { ident, arguments }: &PathSegment) -> bool {
        match self {
            Self::StdOrCore => {
                (ident == "std" || ident == "core") && matches!(arguments, PathArguments::None)
            }
            Self::OptionMod => ident == "option" && matches!(arguments, PathArguments::None),
            Self::Final => ident == "Option" && !matches!(arguments, PathArguments::None),
            Self::Unknown => true,
        }
    }
}

impl OptionCheckerState {
    fn next_states(&self) -> &[Self] {
        match self {
            Self::Final => &[],
            Self::OptionMod => &[Self::Final],
            Self::StdOrCore => &[Self::OptionMod],
            Self::Unknown => &[Self::OptionMod, Self::Final, Self::StdOrCore],
        }
    }
}

pub trait OptionTypeExt {
    fn is_option(&self) -> bool;
}

impl OptionTypeExt for Path {
    fn is_option(&self) -> bool {
        let mut current_state = if self.leading_colon.is_some() {
            OptionCheckerState::StdOrCore
        } else {
            OptionCheckerState::Unknown
        };
        for segment in self.segments.iter() {
            let mut state_changed = false;
            for state in current_state.next_states() {
                if state == segment {
                    current_state = *state;
                    state_changed = true;
                    break;
                }
            }
            if !state_changed {
                return false;
            }
        }
        current_state == OptionCheckerState::Final
    }
}

impl OptionTypeExt for Type {
    fn is_option(&self) -> bool {
        if let Self::Path(TypePath { path, qself: None }) = &self {
            path.is_option()
        } else {
            false
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum ResultCheckerState {
    Unknown = 1,
    StdOrCore = 2,
    ResultMod = 3,
    Final = 4,
}

impl PartialEq<PathSegment> for ResultCheckerState {
    fn eq(&self, PathSegment { ident, arguments }: &PathSegment) -> bool {
        match self {
            Self::StdOrCore => {
                (ident == "std" || ident == "core") && matches!(arguments, PathArguments::None)
            }
            Self::ResultMod => ident == "result" && matches!(arguments, PathArguments::None),
            Self::Final => ident == "Result" && !matches!(arguments, PathArguments::None),
            Self::Unknown => true,
        }
    }
}

impl ResultCheckerState {
    fn next_states(&self) -> &[Self] {
        match self {
            Self::Final => &[],
            Self::ResultMod => &[Self::Final],
            Self::StdOrCore => &[Self::ResultMod],
            Self::Unknown => &[Self::ResultMod, Self::Final, Self::StdOrCore],
        }
    }
}

pub trait ResultTypeExt {
    fn is_result(&self) -> bool;
}

impl ResultTypeExt for Path {
    fn is_result(&self) -> bool {
        let mut current_state = if self.leading_colon.is_some() {
            ResultCheckerState::StdOrCore
        } else {
            ResultCheckerState::Unknown
        };
        for segment in self.segments.iter() {
            let mut state_changed = false;
            for state in current_state.next_states() {
                if state == segment {
                    current_state = *state;
                    state_changed = true;
                    break;
                }
            }
            if !state_changed {
                return false;
            }
        }
        current_state == ResultCheckerState::Final
    }
}

impl ResultTypeExt for Type {
    fn is_result(&self) -> bool {
        if let Self::Path(TypePath { path, qself: None }) = &self {
            path.is_result()
        } else {
            false
        }
    }
}
