use anyhow::Result;

pub enum UrlType {
    Function,
    Log,
}

pub(crate) fn display_url(lambda_name: &str, url_type: UrlType) -> Result<()> {
    let url = match url_type {
        UrlType::Function => format!(
            "https://app.localstack.cloud/inst/default/resources/lambda/functions/{lambda_name}"
        ),
        UrlType::Log => format!(
            "https://app.localstack.cloud/inst/default/resources/cloudwatch/groups/%2Faws%2Flambda%2F{lambda_name}/streams"
        ),
    };

    tracing::info!(url);
    webbrowser::open(&url)?;
    Ok(())
}
