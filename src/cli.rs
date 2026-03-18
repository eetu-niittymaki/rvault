use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rvault")]
#[command(about = "CLI tool for cryptographically saving and retrieving passwords")]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates the database
    Create(CreateCommand),
    /// Add new password to database
    New(NewCommand),
    /// Change name of saved password
    Update(UpdateCommand),
    /// Delete saved password
    Delete(DeleteCommand),
    /// Get names of all saved passwords
    All(AllCommand),
    /// Get specific password by name and save it to clipboard
    Get(GetCommand)
}

#[derive(Parser, Debug, Clone)]
pub struct CreateCommand {
}

#[derive(Parser, Debug, Clone)]
pub struct NewCommand {
    #[arg(short, long, help="Name for password to create")]
    pub name: String,
}

#[derive(Parser, Debug, Clone)]pub struct UpdateCommand {
    #[arg(short, long, help="Name for password to change")]
    pub old_name: String,
    #[arg(short, long, help="Name password is changed to")]
    pub new_name: String,
}

#[derive(Parser, Debug, Clone)]
pub struct DeleteCommand {
    #[arg(short, long, help="Name for password to delete")]
    pub name: String,
}

#[derive(Parser, Debug, Clone)]
pub struct AllCommand {
}

#[derive(Parser, Debug, Clone)]
pub struct GetCommand {
    #[arg(short, long, help="Name for password to get")]
    pub name: String,
}