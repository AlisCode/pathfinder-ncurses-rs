use blocks::case::TypeCase;

#[derive(Copy, Clone)]
pub enum SelectTypeMenuMessage {
    ChangeTypeCase(TypeCase),
}