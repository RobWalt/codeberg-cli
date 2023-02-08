use cod_cli::auth::logout::LogoutArgs;
use cod_paths::token_path;

pub fn logout_user(_args: LogoutArgs) -> anyhow::Result<()> {
    if !inquire::Confirm::new("Logging out deletes your current token. Do you want to proceed?")
        .prompt()?
    {
        return Ok(());
    }

    let token_path = token_path()?;

    std::fs::remove_file(token_path)?;

    Ok(())
}
