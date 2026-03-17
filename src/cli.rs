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
    Create(CreateCommand),
    Add(AddCommand),
    Update(UpdateCommand),
    Delete(DeleteCommand),
    All(AllCommand)
}

#[derive(Parser)]
#[derive(Debug)]
pub struct CreateCommand {
    #[arg(short, long, help="Password for vault")]
    pub password: String,
}

#[derive(Parser)]
#[derive(Debug)]
pub struct AddCommand {
    #[arg(short, long, help="Password for vault")]
    pub password: String,
    #[arg(short, long, help="Name for password to create")]
    pub name: String,
    #[arg(short, long, help="Password")]
    pub word: String
}

#[derive(Parser)]
#[derive(Debug)]
pub struct UpdateCommand {
    #[arg(short, long, help="Password for vault")]
    pub password: String,
    #[arg(short, long, help="Name for password to change")]
    pub old_name: String,
    #[arg(short, long, help="Name password is changed to")]
    pub new_name: String,
}

#[derive(Parser)]
#[derive(Debug)]
pub struct DeleteCommand {
    #[arg(short, long, help="Password for vault")]
    pub password: String,
    #[arg(short, long, help="Name for password to delete")]
    pub name: String,
}

#[derive(Parser)]
#[derive(Debug)]
pub struct AllCommand {
    #[arg(short, long, help="Password for vault")]
    pub password: String,
}