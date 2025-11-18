use gpui::{App, Global};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SwitchMode {
    CardMode,
    TableMode,
}

impl SwitchMode {
    pub fn to_mode(&self, index: usize) -> Self {
        match index {
            0 => Self::CardMode,
            1 => Self::TableMode,
            _ => Self::CardMode,
        }
    }
}

pub struct ActiveSwitchModeState {
    pub mode: SwitchMode,
}

impl Global for ActiveSwitchModeState {}

impl ActiveSwitchModeState {
    pub fn init(cx: &mut App) {
        let this = ActiveSwitchModeState {
            mode: SwitchMode::CardMode,
        };

        cx.set_global(this);
    }
}
