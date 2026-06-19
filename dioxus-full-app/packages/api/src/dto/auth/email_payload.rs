#[derive(Debug, Clone)]
pub struct EmailPayload<T> {
    pub to: String,
    pub subject: String,
    pub html: String,
    pub text: Option<T>,
    pub cc: Option<Vec<String>>,
}
