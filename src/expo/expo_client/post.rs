use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
mod response;

use response::SendPushNotificationResponse;

use crate::{
    error::CustomError,
    object::{ExpoPushErrorTicket, ExpoPushMessage, ExpoPushSuccessTicket, ExpoPushTicket},
};

pub(crate) async fn send_push_notifications(
    client: &reqwest::Client,
    push_message: ExpoPushMessage,
    access_token: Option<&str>,
) -> Result<Vec<ExpoPushTicket>, CustomError> {
    const URL: &str = "https://exp.host/--/api/v2/push/send";
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    if let Some(token) = access_token {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
    }

    match client
        .post(URL)
        .headers(headers)
        .json(&push_message)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                Ok(response
                    .json::<SendPushNotificationResponse>()
                    .await
                    .map_err(|err| {
                        CustomError::DeserializeErr(format!(
                            "Failed to deserialize response: {:?}",
                            err
                        ))
                    })?
                    .data
                    .into_iter()
                    .map(|item| match item {
                        response::SendPushNotificationResponseDataItem::Ok { id } => {
                            ExpoPushTicket::Success(ExpoPushSuccessTicket { id })
                        }
                        response::SendPushNotificationResponseDataItem::Error {
                            message,
                            details,
                        } => ExpoPushTicket::Error(ExpoPushErrorTicket { message, details }),
                    })
                    .collect())
            } else {
                Err(CustomError::ServerErr(format!(
                    "Failed to send request: {:?} ===> 1",
                    response
                )))
            }
        }
        Err(err) => Err(CustomError::ServerErr(format!(
            "Failed to send request: {:?} ===> 2",
            err
        ))),
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[ignore]
    async fn test_valid_post() {
        todo!("test")
    }

    #[tokio::test]
    #[ignore]
    async fn test_invalid_post() {
        todo!("test")
    }
}
