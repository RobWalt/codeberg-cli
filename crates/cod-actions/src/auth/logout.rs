use cod_cli::auth::logout::LogoutArgs;
use cod_paths::token_path;
use cod_render::ui::confirm_with_prompt;

pub fn logout_user(_args: LogoutArgs) -> anyhow::Result<()> {
    if !confirm_with_prompt("Logging out deletes your current token. Do you want to proceed?")? {
        return Ok(());
    }

    let token_path = token_path()?;

    std::fs::remove_file(token_path)?;

    Ok(())
}
