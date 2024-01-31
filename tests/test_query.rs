#[cfg(test)]
mod tests {

    use anyhow::Result;
    use reqwest::StatusCode;
    use urlencoding::encode;
    static SERVER_URL: &str = "http://localhost:9341";

    #[tokio::test]
    async fn test_query() -> Result<()> {
        let hc = httpc_test::new_client(SERVER_URL)?;
        let query =
            encode("Neon Genesis Evangelion 1-26 Complete (Dual Audio) [BDRip 720p]").into_owned();
        let response = hc.do_get(&format!("/query/{}", query)).await?;
        response.print().await?;

        assert!(response.status() == StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn test_send_magnet() -> Result<()> {
        let hc = httpc_test::new_client(SERVER_URL)?;
        let query =
            encode("magnet:?xt=urn:btih:7997B3932B3017C3994B527BBFB81D8D9ECAA9D9").into_owned();
        let json_payload = format!(r#"{{ "magnet": "{}" }}"#, query);
        let content_type = "application/x-www-form-urlencoded";

        let res = hc
            .do_post("/send-to-transmission/", (json_payload, content_type))
            .await?;
        res.print().await?;

        assert!(res.status() == StatusCode::OK);

        Ok(())
    }
}
