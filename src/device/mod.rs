mod model;

pub use model::*;

/* #[cfg(test)]
mod tests {
    use dotenvy::dotenv;

    use self::uisp::UISP_INSTANCE;

    use super::*;

    #[actix_web::test]
    async fn test_fetch_device_list() -> Result<(), ApiError> {
        dotenv().ok();
        let list = UISP_INSTANCE.fetch_device_list().await?;

        // List should be non-empty
        assert!(list.len() > 0);
        Ok(())
    }
} */
