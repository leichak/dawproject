// this is struct that represents nameable and referencable

struct Identifiable {
    // it will be in the struct anyways
    pub name: String,    // attribute
    pub color: String,   // att
    pub comment: String, // att
    pub id: String,
}

pub trait Identifiable {
    pub fn reset_id(&mut self) {
        self.id = "";
    }
}
