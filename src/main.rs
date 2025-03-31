use open::that as open_file;
use anyhow::{Context, Result};
use colored::*;
use std::{
    fs,
    path::{Path, PathBuf},
};
use rustyline::DefaultEditor;

struct FileExplorer {
    current_dir: PathBuf,
}

impl FileExplorer {
    fn new() -> Result<Self> {
        Ok(Self {
            current_dir: std::env::current_dir()?,
        })
    }

    fn change_directory(&mut self, path: &Path) -> Result<()> {
        let new_path = self.current_dir.join(path);
        if new_path.is_dir() {
            std::env::set_current_dir(&new_path)?;
            self.current_dir = new_path.canonicalize()?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Directory not found: {}", new_path.display()))
        }
    }

    fn create_file(&self, filename: &str) -> Result<()> {
        let path = self.current_dir.join(filename);
        fs::File::create(&path)
            .with_context(|| format!("Failed to create file: {}", path.display()))?;
        Ok(())
    }

    fn delete_path(&self, path: &str) -> Result<()> {
        let target = self.current_dir.join(path);
        if target.is_dir() {
            fs::remove_dir_all(&target)
        } else {
            fs::remove_file(&target)
        }.with_context(|| format!("Failed to delete: {}", target.display()))
    }

    fn open_file(&self, filename: &str) -> Result<()> {
        let path = self.current_dir.join(filename);
        open_file(&path)
            .with_context(|| format!("Failed to open: {}", path.display()))
    }

    fn list_files_with_color(&self) -> Result<Vec<String>> {
        let entries = fs::read_dir(&self.current_dir)?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    let path = e.path();
                    let name = e.file_name().into_string().ok()?;
                    Some(if path.is_dir() {
                        name.blue().bold().to_string()
                    } else {
                        name.yellow().to_string()
                    })
                })
            })
            .collect();
        Ok(entries)
    }
}

fn main() -> Result<()> {
    let mut explorer = FileExplorer::new()?;
    let mut rl = DefaultEditor::new()?;

    println!("{}", " Rust File Explorer ".bold().on_cyan().black());
    println!("{}", "━".repeat(40).cyan());
    print_help();

    loop {
        let prompt = format!("{} {} ",
                             "➤".green().bold(),
                             explorer.current_dir.display().to_string().cyan()
        );

        let input = match rl.readline(&prompt) {
            Ok(line) => line,
            Err(_) => break,
        };
        rl.add_history_entry(&input).ok();

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "cd" => handle_cd(&mut explorer, &parts),
            "ls" => handle_ls(&explorer),
            "open" => handle_open(&explorer, &parts),
            "delete" => handle_delete(&explorer, &parts),
            "create" => handle_create(&explorer, &parts),
            "help" => print_help(),
            "exit" => break,
            cmd => println!("{}: {}", "Unknown command".red().bold(), cmd.yellow()),
        }
    }
    Ok(())
}

// Colorized command handlers
fn handle_cd(explorer: &mut FileExplorer, parts: &[&str]) {
    if parts.len() < 2 {
        println!("{}", "Usage: cd <directory>".yellow());
        return;
    }
    match explorer.change_directory(Path::new(parts[1])) {
        Ok(_) => println!("{} {}", "✓".green(),
                          format!("Changed directory to {}", explorer.current_dir.display()).cyan()),
        Err(e) => println!("{} {}", "✗".red(), format!("Error: {}", e).red()),
    }
}

fn handle_ls(explorer: &FileExplorer) {
    match explorer.list_files_with_color() {
        Ok(files) => {
            println!("{} {}", "📂".cyan(),
                     format!("Contents of {}:", explorer.current_dir.display()).bold());
            for file in files {
                println!("  {}", file);
            }
        }
        Err(e) => println!("{} {}", "✗".red(), format!("Error: {}", e).red()),
    }
}

fn handle_open(explorer: &FileExplorer, parts: &[&str]) {
    if parts.len() < 2 {
        println!("{}", "Usage: open <filename>".yellow());
        return;
    }
    match explorer.open_file(parts[1]) {
        Ok(_) => println!("{} {}", "✓".green(),
                          format!("Opened {}", parts[1]).cyan()),
        Err(e) => println!("{} {}", "✗".red(), format!("Error: {}", e).red()),
    }
}

fn handle_delete(explorer: &FileExplorer, parts: &[&str]) {
    if parts.len() < 2 {
        println!("{}", "Usage: delete <filename>".yellow());
        return;
    }
    match explorer.delete_path(parts[1]) {
        Ok(_) => println!("{} {}", "✓".green(),
                          format!("Deleted {}", parts[1]).cyan()),
        Err(e) => println!("{} {}", "✗".red(), format!("Error: {}", e).red()),
    }
}

fn handle_create(explorer: &FileExplorer, parts: &[&str]) {
    if parts.len() < 2 {
        println!("{}", "Usage: create <filename>".yellow());
        return;
    }
    match explorer.create_file(parts[1]) {
        Ok(_) => println!("{} {}", "✓".green(),
                          format!("Created {}", parts[1]).cyan()),
        Err(e) => println!("{} {}", "✗".red(), format!("Error: {}", e).red()),
    }
}

fn print_help() {
    println!("\n{}", "Available commands:".bold().underline());
    println!("  {} {}", "cd".cyan().bold(), "<directory>".yellow());
    println!("  {}  {}", "ls".cyan().bold(), "List directory contents".white());
    println!("  {} {}", "open".cyan().bold(), "<file>".yellow());
    println!("  {} {}", "delete".cyan().bold(), "<file>".yellow());
    println!("  {} {}", "create".cyan().bold(), "<file>".yellow());
    println!("  {}  {}", "help".cyan().bold(), "Show this help".white());
    println!("  {}  {}\n", "exit".cyan().bold(), "Exit the program".white());
}