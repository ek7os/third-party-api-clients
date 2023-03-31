use anyhow::Result;

#[async_trait::async_trait]
pub trait JobOps {
    /// Send a plain text email.
    ///
    /// This is a nicer experience than using `post`.
    async fn post(&self, b: bytes::Bytes) -> Result<crate::types::Job>;
}

#[async_trait::async_trait]
impl JobOps for crate::jobs::Jobs {
    /// Create a job.
    async fn post(&self, b: bytes::Bytes) -> Result<crate::types::Job> {
        let url = self.client.url("/jobs", None);
        let form = reqwest::multipart::Form::new()
            .part(
                "media",
                reqwest::multipart::Part::bytes(b.to_vec())
                    .mime_str("video/mp4")
                    .unwrap()
                    .file_name("testing.mp4"),
            )
            .text("options", "{}");

        self.client.post_form(&url, form).await
    }
}
