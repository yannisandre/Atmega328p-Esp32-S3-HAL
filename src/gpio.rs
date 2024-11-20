// ce module permet de définir des structures communes à implémenter pour les deux target pour le GPIO

#[derive(PartialEq)] // définit la gestion du == entre deux instances de PinMode
pub enum PinMode {
    Input,
    Output,
}

pub trait Gpio { // implémenter Gpio force le fait de définir les trois fonctions suivantes
    fn set_mode(&self, mode: PinMode);
    fn write(&self, value: bool);
    fn read(&self) -> bool;
}
