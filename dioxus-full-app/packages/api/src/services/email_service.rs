use lettre::{
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
    message::{Mailbox, Message, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
};

use crate::{dto::auth::email_payload::EmailPayload, errors::app_error::AppError};

#[derive(Clone)]
pub struct EmailService {
    transporter: AsyncSmtpTransport<Tokio1Executor>,
    from: String,
}

impl EmailService {
    pub fn new() -> Result<Self, AppError> {
        let host = std::env::var("EMAIL_HOST").map_err(|e| AppError::Internal(e.to_string()))?;
        let port_email = std::env::var("EMAIL_PORT")
            .map_err(|e| AppError::Internal(e.to_string()))?
            .parse::<u16>()
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let username =
            std::env::var("EMAIL_HOST_USER").map_err(|e| AppError::Internal(e.to_string()))?;

        let password =
            std::env::var("EMAIL_HOST_PASSWORD").map_err(|e| AppError::Internal(e.to_string()))?;

        let from = std::env::var("EMAIL_FROM").map_err(|e| AppError::Internal(e.to_string()))?;

        let credentials = Credentials::new(username, password);

        let transporter = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&host)
            .map_err(|e| AppError::Internal(e.to_string()))?
            .credentials(credentials)
            .port(port_email)
            .build();

        Ok(Self { transporter, from })
    }

    pub async fn verify_connection(&self) -> Result<(), AppError> {
        self.transporter
            .test_connection()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }
    pub async fn send_email(&self, payload: EmailPayload<String>) -> Result<(), AppError> {
        let mut builder = Message::builder()
            .from(
                self.from
                    .parse::<Mailbox>()
                    .map_err(|e| AppError::Internal(e.to_string()))?,
            )
            .to(payload
                .to
                .parse::<Mailbox>()
                .map_err(|e| AppError::Internal(e.to_string()))?)
            .subject(payload.subject);

        if let Some(cc_list) = payload.cc {
            for cc in cc_list {
                builder = builder.cc(cc
                    .parse::<Mailbox>()
                    .map_err(|e| AppError::Internal(e.to_string()))?);
            }
        }

        let email = if let Some(text) = payload.text {
            builder
                .multipart(
                    MultiPart::alternative()
                        .singlepart(SinglePart::plain(text))
                        .singlepart(SinglePart::html(payload.html)),
                )
                .map_err(|e| AppError::Internal(e.to_string()))?
        } else {
            builder
                .singlepart(SinglePart::html(payload.html))
                .map_err(|e| AppError::Internal(e.to_string()))?
        };

        self.transporter
            .send(email)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }
}
