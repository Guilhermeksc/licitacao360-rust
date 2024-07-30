// src/controller/menu_superior.rs:

use druid::Env;
use druid::{Menu, MenuItem, LocalizedString, Data, WindowId, Selector};

const MENU_FILE_NEW: Selector = Selector::new("menu-file-new");
const MENU_FILE_OPEN: Selector = Selector::new("menu-file-open");
const MENU_FILE_SAVE: Selector = Selector::new("menu-file-save");
const MENU_CONFIG_SETTINGS: Selector = Selector::new("menu-config-settings");
const MENU_UTILITIES: Selector = Selector::new("menu-utilities");
const MENU_ABOUT: Selector = Selector::new("menu-about");

pub fn build_menu<T: Data>(_window_id: Option<WindowId>, _app_state: &T, _env: &Env) -> Menu<T> {
    let file_menu = Menu::new(LocalizedString::new("Arquivo"))
        .entry(MenuItem::new(LocalizedString::new("Novo")).command(MENU_FILE_NEW))
        .entry(MenuItem::new(LocalizedString::new("Abrir")).command(MENU_FILE_OPEN))
        .entry(MenuItem::new(LocalizedString::new("Salvar")).command(MENU_FILE_SAVE));

    let config_menu = Menu::new(LocalizedString::new("Configurações"))
        .entry(MenuItem::new(LocalizedString::new("Configurações")).command(MENU_CONFIG_SETTINGS));

    let utilities_menu = Menu::new(LocalizedString::new("Utilidades"))
        .entry(MenuItem::new(LocalizedString::new("Utilidades")).command(MENU_UTILITIES));

    let about_menu = Menu::new(LocalizedString::new("Sobre"))
        .entry(MenuItem::new(LocalizedString::new("Sobre")).command(MENU_ABOUT));

    Menu::empty()
        .entry(file_menu)
        .entry(config_menu)
        .entry(utilities_menu)
        .entry(about_menu)
}

pub struct Delegate;

impl<T: Data> druid::AppDelegate<T> for Delegate {
    fn command(&mut self, _ctx: &mut druid::DelegateCtx, _target: druid::Target, cmd: &druid::Command, _data: &mut T, _env: &Env) -> druid::Handled {
        match cmd.get(MENU_FILE_NEW) {
            Some(_) => {
                println!("Novo arquivo");
                druid::Handled::Yes
            }
            None => druid::Handled::No,
        }
    }
}
