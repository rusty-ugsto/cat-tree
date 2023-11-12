pub trait Handler {
    fn execute(&self) -> Result<(), String>;
}
