use crate::frontend::auth::logout::LogoutArgs;
use crate::paths::token_path;

pub fn logout(args: LogoutArgs) -> anyhow::Result<()> {
    if !dialoguer::Confirm::new()
        .with_prompt("Logging out deletes your current token. Do you want to proceed?")
        .interact()?
    {
        return Ok(());
    }

    let token_path = token_path()?;

    std::fs::remove_file(token_path)?;

    Ok(())
}
