use surrealdb::opt::RecordId;

pub trait LinkId where {
    fn new_id(&self) -> RecordId;
}

impl LinkId for RecordId {
    fn new_id(&self) -> RecordId {
        self.new_id()
    }
}
