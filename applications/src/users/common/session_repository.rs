pub trait SessionRepository {
    fn find(&self);
    fn insert(&self);
    fn delete(&self);
}