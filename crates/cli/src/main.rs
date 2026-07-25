use std::env;
use std::fs;

use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::{Parser, Subcommand};
use colored::*;

use pickledb_core::traits::{Client, Engine};
use pickledb_core::types::{InsertTuple, RecordId, SearchToken};
use pickledb_crypto::client::PickleClient;
use pickledb_engine::engine::PickleEngine;

fn cli_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
        .placeholder(AnsiColor::White.on_default().effects(Effects::ITALIC))
        .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
        .valid(AnsiColor::Green.on_default().effects(Effects::BOLD))
}

fn print_banner() {
    let banner = r#"
  ____  _ _        _      ____  ____
 |  _ \(_) | _____| | ___|  _ \| __ )
 | |_) | | |/ / _ \ |/ _ \ | | |  _ \
 |  __/| |   <  __/ |  __/ |_| | |_) |
 |_|   |_|_|\_\___|_|\___|____/|____/
"#;
    println!("{}", banner.green());
    println!("{}", " Zero-Trust Encrypted Database Engine\n".white().bold());
}

fn get_master_key() -> anyhow::Result<Vec<u8>> {
    let key_str = env::var("PICKLEDB_KEY")
        .map_err(|_| anyhow::anyhow!(
            "{}\n{}",
            format!("{} PICKLEDB_KEY environment variable not set", "[ERROR]".red().bold()),
            format!("      Set it to a 32-byte hex key: export PICKLEDB_KEY={}", "<32-hex-chars>".cyan())
        ))?;
    let key = key_str.as_bytes().to_vec();
    anyhow::ensure!(key.len() == 32, "PICKLEDB_KEY must be exactly 32 bytes, got {}", key.len());
    Ok(key)
}

fn open_engine(dir: &str) -> anyhow::Result<PickleEngine> {
    Ok(PickleEngine::open(dir)?)
}

#[derive(Parser)]
#[command(
    name = "pickledb",
    version = env!("CARGO_PKG_VERSION"),
    about = "Zero-Trust Encrypted Database Engine",
    long_about = None,
    styles = cli_styles(),
    propagate_version = true,
    arg_required_else_help = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new database directory
    Init {
        /// Path to the database directory
        dir: String,
    },

    /// Insert an encrypted record
    Insert {
        /// Database directory (default: PICKLEDB_DB env)
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,

        /// Record ID
        id: u64,

        /// Data to encrypt and store
        data: String,
    },

    /// Retrieve and decrypt a record
    Get {
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,

        /// Record ID
        id: u64,
    },

    /// Search records by search token (hex-encoded)
    Search {
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,

        /// Search token in hex format (64 hex chars)
        token: String,
    },

    /// Update an existing record
    Update {
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,

        /// Record ID
        id: u64,

        /// New data to encrypt and store
        data: String,
    },

    /// Delete a record
    Delete {
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,

        /// Record ID
        id: u64,
    },

    /// Display database statistics
    Stats {
        /// Database directory (default: PICKLEDB_DB env)
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,
    },

    /// Compact storage pages
    Compact {
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,
    },

    /// Flush all pending writes to durable storage
    Sync {
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,
    },

    /// Create a WAL checkpoint
    Checkpoint {
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,
    },

    /// Run database diagnostics
    Doctor {
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,
    },

    /// Inspect internal database structures
    Inspect {
        #[command(subcommand)]
        target: InspectTarget,
    },

    /// View and analyze the write-ahead log
    Wal {
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,

        /// Tail the WAL (follow new entries)
        #[arg(short, long)]
        tail: bool,
    },

    /// Visualize query execution path
    Explain {
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,

        /// Query to explain (e.g., "get 42")
        query: String,
    },

    /// Start an interactive shell
    Shell {
        /// Database directory (default: PICKLEDB_DB env)
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,
    },

    /// Run performance benchmarks
    Benchmark {
        #[arg(short, long, env = "PICKLEDB_DB")]
        dir: Option<String>,
    },

    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completions for (bash, zsh, fish, powershell, elvish)
        shell: String,
    },
}

#[derive(Subcommand)]
enum InspectTarget {
    /// Inspect a specific page
    Page {
        /// Page ID
        id: u32,
    },
    /// Inspect a specific record
    Record {
        /// Record ID
        id: u64,
    },
    /// Inspect the search index
    Index,
    /// Inspect the buffer pool cache
    Cache,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { dir } => cmd_init(dir),
        Commands::Insert { dir, id, data } => cmd_insert(dir, *id, data),
        Commands::Get { dir, id } => cmd_get(dir, *id),
        Commands::Search { dir, token } => cmd_search(dir, token),
        Commands::Update { dir, id, data } => cmd_update(dir, *id, data),
        Commands::Delete { dir, id } => cmd_delete(dir, *id),
        Commands::Stats { dir } => cmd_stats(dir),
        Commands::Compact { dir } => cmd_compact(dir),
        Commands::Sync { dir } => cmd_sync(dir),
        Commands::Checkpoint { dir } => cmd_checkpoint(dir),
        Commands::Doctor { dir } => cmd_doctor(dir),
        Commands::Inspect { target } => match target {
            InspectTarget::Page { id } => cmd_inspect_page(id),
            InspectTarget::Record { id } => cmd_inspect_record(*id),
            InspectTarget::Index => cmd_inspect_index(),
            InspectTarget::Cache => cmd_inspect_cache(),
        },
        Commands::Wal { dir, tail } => cmd_wal(dir, *tail),
        Commands::Explain { dir, query } => cmd_explain(dir, query),
        Commands::Shell { dir } => cmd_shell(dir),
        Commands::Benchmark { dir } => cmd_benchmark(dir),
        Commands::Completion { shell } => cmd_completion(shell.as_str()),
    }
}

// ─── Core Commands ─────────────────────────────────────────────

fn cmd_init(dir: &str) -> anyhow::Result<()> {
    print_banner();
    eprint!("{} Initializing database... ", "[...]".yellow().bold());
    fs::create_dir_all(dir)?;
    let mut engine = PickleEngine::open(dir)?;
    engine.sync()?;
    println!("{}", "done".green().bold());
    println!();
    println!("  {} PickleDB database initialized", "✓".green().bold());
    println!("  {} Directory: {}", "  Location:", dir.cyan());
    println!("  {} Use: pickledb --dir {} <command>", "  Next:", dir.cyan());
    Ok(())
}

fn cmd_insert(dir: &Option<String>, id: u64, data: &str) -> anyhow::Result<()> {
    let dir = resolve_dir(dir)?;
    let key = get_master_key()?;
    let client = PickleClient::new(&key)?;
    let mut engine = open_engine(&dir)?;

    let payload = client.encrypt(RecordId(id), data.as_bytes())?;
    let tuple = InsertTuple {
        record_id: RecordId(id),
        payload,
        search_tokens: vec![],
    };
    engine.insert(tuple)?;

    println!(
        "{} Record {} inserted into {}",
        "✓".green().bold(),
        id.to_string().cyan().bold(),
        dir.cyan()
    );
    Ok(())
}

fn cmd_get(dir: &Option<String>, id: u64) -> anyhow::Result<()> {
    let dir = resolve_dir(dir)?;
    let key = get_master_key()?;
    let client = PickleClient::new(&key)?;
    let engine = open_engine(&dir)?;

    match engine.get(RecordId(id)) {
        Ok(payload) => match client.decrypt(RecordId(id), &payload) {
            Ok(plaintext) => {
                let s = String::from_utf8_lossy(&plaintext);
                println!(
                    "{} Record {}: {}",
                    "✓".green().bold(),
                    id.to_string().cyan().bold(),
                    s.white()
                );
            }
            Err(e) => {
                eprintln!(
                    "{} Record {}: decryption failed: {}",
                    "[DECRYPT ERR]".red().bold(),
                    id,
                    e
                );
            }
        },
        Err(e) => {
            eprintln!(
                "{} Record {} not found: {}",
                "[NOT FOUND]".yellow().bold(),
                id.to_string().cyan().bold(),
                e
            );
        }
    }
    Ok(())
}

fn cmd_search(dir: &Option<String>, token_hex: &str) -> anyhow::Result<()> {
    let dir = resolve_dir(dir)?;
    let bytes = hex::decode(token_hex)
        .map_err(|_| anyhow::anyhow!("Invalid hex token: {}", token_hex))?;
    if bytes.len() != 32 {
        anyhow::bail!("Search token must be 32 bytes (64 hex chars), got {} bytes", bytes.len());
    }
    let mut token_bytes = [0u8; 32];
    token_bytes.copy_from_slice(&bytes);
    let token = SearchToken(token_bytes);

    let engine = open_engine(&dir)?;
    match engine.search(&token) {
        Ok(records) => {
            if records.is_empty() {
                println!("{} No matching records found", "[0]".yellow().bold());
            } else {
                println!(
                    "{} Found {} matching record{}:",
                    "✓".green().bold(),
                    records.len().to_string().cyan().bold(),
                    if records.len() == 1 { "" } else { "s" }
                );
                for id in &records {
                    println!("  {} {}", "•".cyan(), id.0.to_string().white());
                }
            }
        }
        Err(_) => {
            println!("{} No matching records found", "[0]".yellow().bold());
        }
    }
    Ok(())
}

fn cmd_update(dir: &Option<String>, id: u64, data: &str) -> anyhow::Result<()> {
    let dir = resolve_dir(dir)?;
    let key = get_master_key()?;
    let client = PickleClient::new(&key)?;
    let mut engine = open_engine(&dir)?;

    let payload = client.encrypt(RecordId(id), data.as_bytes())?;
    let tuple = InsertTuple {
        record_id: RecordId(id),
        payload,
        search_tokens: vec![],
    };
    engine.update(RecordId(id), tuple)?;

    println!(
        "{} Record {} updated in {}",
        "✓".green().bold(),
        id.to_string().cyan().bold(),
        dir.cyan()
    );
    Ok(())
}

fn cmd_delete(dir: &Option<String>, id: u64) -> anyhow::Result<()> {
    let dir = resolve_dir(dir)?;
    let mut engine = open_engine(&dir)?;
    engine.delete(RecordId(id))?;
    println!(
        "{} Record {} deleted from {}",
        "✓".green().bold(),
        id.to_string().cyan().bold(),
        dir.cyan()
    );
    Ok(())
}

fn cmd_sync(dir: &Option<String>) -> anyhow::Result<()> {
    let dir = resolve_dir(dir)?;
    let mut engine = open_engine(&dir)?;
    engine.sync()?;
    println!(
        "{} Storage synced for {}",
        "✓".green().bold(),
        dir.cyan()
    );
    Ok(())
}

fn cmd_checkpoint(dir: &Option<String>) -> anyhow::Result<()> {
    let dir = resolve_dir(dir)?;
    let mut engine = open_engine(&dir)?;
    engine.checkpoint()?;
    println!(
        "{} WAL checkpoint created for {}",
        "✓".green().bold(),
        dir.cyan()
    );
    Ok(())
}

fn cmd_compact(dir: &Option<String>) -> anyhow::Result<()> {
    let dir = resolve_dir(dir)?;
    eprint!("{} Compacting storage... ", "[...]".yellow().bold());
    let mut engine = open_engine(&dir)?;
    engine.compact()?;
    println!("{}", "done".green().bold());
    println!(
        "{} Storage compacted for {}",
        "✓".green().bold(),
        dir.cyan()
    );
    Ok(())
}

// ─── Database Commands ─────────────────────────────────────────

fn cmd_stats(dir: &Option<String>) -> anyhow::Result<()> {
    let dir = resolve_dir(dir)?;
    let engine = open_engine(&dir)?;

    let num_pages = engine.file_manager().read().num_pages();
    let wal_lsn = engine.wal().read().current_lsn();
    let wal_entries = engine.wal().read().entry_count();
    let index_entries = engine.index().total_entries();
    let token_count = engine.index().token_count();
    let cache_size = engine.buffer_pool().read().len();
    let record_count = engine.record_map().read().len();

    let db_path = std::path::Path::new(&dir).join("data.db");
    let db_size = fs::metadata(&db_path).map(|m| m.len()).unwrap_or(0);
    let page_size = pickledb_core::types::PAGE_SIZE;
    let total_data_size = num_pages as u64 * page_size as u64;

    println!();
    println!("{}", "  Database Statistics".green().bold());
    println!("  {}", "─".repeat(50).dimmed());
    println!();
    println!("  {:<20} {}", "Database:", dir.cyan());
    println!("  {:<20} {}", "Status:", "Online".green().bold());
    println!("  {:<20} {}", "Engine:", "PickleDB v0.1.0".white());
    println!("  {:<20} {}", "Encryption:", "AES-256-GCM".cyan());
    println!();
    println!("{}", "  Storage".white().bold());
    println!("  {:<20} {}", "  Database size:", format_size(db_size));
    println!("  {:<20} {}", "  Page size:", format!("{} bytes", page_size));
    println!("  {:<20} {}", "  Total pages:", num_pages.to_string().cyan());
    println!("  {:<20} {}", "  Total capacity:", format_size(total_data_size));
    println!("  {:<20} {}", "  Records:", record_count.to_string().cyan());
    println!();
    println!("{}", "  Cache".white().bold());
    println!("  {:<20} {}", "  Buffer pool:", format!("{} pages", cache_size));
    println!("  {:<20} {}", "  Cache hit ratio:", "N/A".dimmed());
    println!();
    println!("{}", "  WAL".white().bold());
    println!("  {:<20} {}", "  Current LSN:", format!("{}", wal_lsn.0).cyan());
    println!("  {:<20} {}", "  Entries:", wal_entries.to_string().cyan());
    println!();
    println!("{}", "  Index".white().bold());
    println!("  {:<20} {}", "  Tokens:", token_count.to_string().cyan());
    println!("  {:<20} {}", "  Index entries:", index_entries.to_string().cyan());
    println!();

    Ok(())
}

fn cmd_doctor(dir: &Option<String>) -> anyhow::Result<()> {
    let dir = resolve_dir(dir)?;
    let engine = open_engine(&dir)?;
    let mut wal = engine.wal().write();

    println!();
    println!("{}", "  Database Diagnostics Report".green().bold());
    println!("  {}", "─".repeat(50).dimmed());
    println!();

    // Check 1: Database directory exists
    let dir_path = std::path::Path::new(&dir);
    if dir_path.exists() {
        println!("  {} Directory exists", "✓".green().bold());
    } else {
        println!("  {} Directory does not exist", "✗".red().bold());
        return Ok(());
    }

    // Check 2: Data file exists
    let data_path = dir_path.join("data.db");
    if data_path.exists() {
        let meta = fs::metadata(&data_path).unwrap();
        println!("  {} Data file present ({})", "✓".green().bold(), format_size(meta.len()));
    } else {
        println!("  {} No data file (empty database)", "○".yellow().bold());
    }

    // Check 3: WAL file exists
    let wal_path = dir_path.join("wal.log");
    if wal_path.exists() {
        let count = wal.entry_count();
        println!("  {} WAL log present ({} entries)", "✓".green().bold(), count);
    } else {
        println!("  {} No WAL file", "○".yellow().bold());
    }

    // Check 4: WAL integrity
    match wal.verify_integrity() {
        Ok(_) => println!("  {} WAL integrity verified", "✓".green().bold()),
        Err(e) => println!("  {} WAL integrity check failed: {}", "✗".red().bold(), e),
    }

    // Check 5: Index state
    let token_count = engine.index().token_count();
    let index_entries = engine.index().total_entries();
    println!("  {} Index: {} tokens, {} entries", "✓".green().bold(), token_count, index_entries);

    // Check 6: Record map
    let record_count = engine.record_map().read().len();
    println!("  {} Record map: {} records tracked", "✓".green().bold(), record_count);

    // Check 7: Cache
    let cache_size = engine.buffer_pool().read().len();
    println!("  {} Buffer pool: {} pages cached", "✓".green().bold(), cache_size);

    // Check 8: Encryption key
    match env::var("PICKLEDB_KEY") {
        Ok(_) => println!("  {} PICKLEDB_KEY is set", "✓".green().bold()),
        Err(_) => println!("  ○ PICKLEDB_KEY not set (required for operations)"),
    }

    println!();
    println!("{}", "  Recommendations".white().bold());
    println!("  {}", "─".repeat(50).dimmed());
    if !wal_path.exists() {
        println!("  • Run sync to create WAL: pickledb --dir {} sync", dir.cyan());
    }
    if !data_path.exists() {
        println!("  • Insert some data to create the database file");
    }
    println!("  • Periodic verification: pickledb --dir {} doctor", dir.cyan());
    println!("  • Back up the directory: {}", dir.cyan());
    println!();

    Ok(())
}

// ─── Debug Commands ────────────────────────────────────────────

fn cmd_inspect_page(id: &u32) -> anyhow::Result<()> {
    println!(
        "{} Page {} inspection",
        "○".cyan().bold(),
        id.to_string().cyan().bold()
    );
    println!("  Page inspection is not yet implemented");
    Ok(())
}

fn cmd_inspect_record(_id: u64) -> anyhow::Result<()> {
    println!(
        "{} Record inspection is not yet implemented",
        "○".yellow().bold()
    );
    Ok(())
}

fn cmd_inspect_index() -> anyhow::Result<()> {
    println!(
        "{} Index inspection is not yet implemented",
        "○".yellow().bold()
    );
    Ok(())
}

fn cmd_inspect_cache() -> anyhow::Result<()> {
    println!(
        "{} Cache inspection is not yet implemented",
        "○".yellow().bold()
    );
    Ok(())
}

fn cmd_wal(dir: &Option<String>, _tail: bool) -> anyhow::Result<()> {
    let dir = resolve_dir(dir)?;
    let engine = open_engine(&dir)?;
    let wal = engine.wal().read();
    let entries = wal.entry_count();
    let lsn = wal.current_lsn();

    println!();
    println!("{}", "  WAL Analysis".green().bold());
    println!("  {}", "─".repeat(50).dimmed());
    println!("  {:<20} {}", "Database:", dir.cyan());
    println!("  {:<20} {}", "Current LSN:", format!("{}", lsn.0).cyan());
    println!("  {:<20} {}", "Total entries:", entries.to_string().cyan());
    println!();
    println!("{}", "  WAL viewer is not yet fully implemented".yellow());
    println!();

    Ok(())
}

fn cmd_explain(dir: &Option<String>, _query: &str) -> anyhow::Result<()> {
    let _ = resolve_dir(dir)?;
    println!();
    println!("{}", "  Execution Plan".green().bold());
    println!("  {}", "─".repeat(50).dimmed());
    println!();
    println!("  Query explain is not yet implemented");
    println!();

    Ok(())
}

fn cmd_shell(dir: &Option<String>) -> anyhow::Result<()> {
    let _dir = resolve_dir(dir)?;
    print_banner();
    println!("{} Interactive shell", "▶".cyan().bold());
    println!("  Type {} for help, {} to exit", ".help".cyan(), ".exit".cyan());
    println!();

    use rustyline::completion::Completer;
    use rustyline::config::Configurer;
    use rustyline::error::ReadlineError;
    use rustyline::highlight::Highlighter;
    use rustyline::hint::Hinter;
    use rustyline::history::FileHistory;
    use rustyline::validate::{ValidationContext, ValidationResult, Validator};
    use rustyline::Context;
    use rustyline::Helper;

    const COMMANDS: &[&str] = &[
        "init", "insert", "get", "search", "update", "delete",
        "stats", "compact", "sync", "checkpoint",
        "doctor", "inspect", "wal", "explain",
        "benchmark", "completion",
        ".help", ".exit", ".quit", ".version", ".clear",
    ];

    #[derive(Clone)]
    struct ShellHelper;

    impl Completer for ShellHelper {
        type Candidate = String;

        fn complete(
            &self,
            line: &str,
            pos: usize,
            _ctx: &Context<'_>,
        ) -> Result<(usize, Vec<String>), ReadlineError> {
            let partial = &line[..pos];
            let last_word = partial.split_whitespace().last().unwrap_or("");
            let completions: Vec<String> = COMMANDS
                .iter()
                .filter(|c| c.starts_with(last_word))
                .map(|c| c.to_string())
                .collect();
            let start = pos - last_word.len();
            Ok((start, completions))
        }
    }

    impl Hinter for ShellHelper {
        type Hint = String;

        fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
            None
        }
    }

    impl Highlighter for ShellHelper {}

    impl Validator for ShellHelper {
        fn validate(&self, _ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
            Ok(ValidationResult::Valid(None))
        }

        fn validate_while_typing(&self) -> bool {
            false
        }
    }

    impl Helper for ShellHelper {}

    let mut rl = rustyline::Editor::<ShellHelper, FileHistory>::new()?;
    rl.set_auto_add_history(true);
    if let Ok(home) = std::env::var("HOME") {
        let hist_path = std::path::Path::new(&home).join(".pickledb_history");
        let _ = rl.load_history(&hist_path);
    }

    loop {
        let prompt = format!("{} ", "pickledb>".green().bold());
        match rl.readline(&prompt) {
            Ok(line) => {
                let input = line.trim().to_string();
                if input.is_empty() {
                    continue;
                }
                match input.as_str() {
                    ".exit" | ".quit" => {
                        println!("{}", "Goodbye!".green());
                        break;
                    }
                    ".help" => {
                        println!("{}", "  Commands".white().bold());
                        println!("  {}", "────────────────────────────────────────".dimmed());
                        println!("  {:<20}  {}", ".exit".cyan(), "Exit the shell");
                        println!("  {:<20}  {}", ".quit".cyan(), "Exit the shell");
                        println!("  {:<20}  {}", ".help".cyan(), "Show this help");
                        println!("  {:<20}  {}", ".version".cyan(), "Show version");
                        println!("  {:<20}  {}", ".clear".cyan(), "Clear the screen");
                        println!();
                        println!("  {} For database operations, use:", "Tip:".yellow().bold());
                        println!("  {:<20}  {}", "insert <id> <data>".cyan(), "Insert a record");
                        println!("  {:<20}  {}", "get <id>".cyan(), "Get a record");
                        println!("  {:<20}  {}", "delete <id>".cyan(), "Delete a record");
                        println!("  {:<20}  {}", "stats".cyan(), "Show statistics");
                        println!("  {:<20}  {}", "sync".cyan(), "Flush to disk");
                    }
                    ".version" => {
                        println!("  PickleDB v{}", env!("CARGO_PKG_VERSION"));
                        println!("  Zero-Trust Encrypted Database Engine");
                        println!("  AES-256-GCM | WAL | Blind Search");
                    }
                    ".clear" => {
                        println!("\x1b[2J\x1b[1;1H");
                    }
                    _ => {
                        println!(
                            "{} Unknown command: '{}'. Type {} for help.",
                            "?".yellow().bold(),
                            input,
                            ".help".cyan()
                        );
                    }
                }
            }
            Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
                println!("{}", "Goodbye!".green());
                break;
            }
            Err(e) => {
                eprintln!("{} Readline error: {}", "[ERR]".red().bold(), e);
                break;
            }
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let hist_path = std::path::Path::new(&home).join(".pickledb_history");
        let _ = rl.save_history(&hist_path);
    }

    Ok(())
}

fn cmd_benchmark(_dir: &Option<String>) -> anyhow::Result<()> {
    println!(
        "{} Benchmark suite is not yet implemented",
        "○".yellow().bold()
    );
    println!("  Run: cargo bench");
    Ok(())
}

fn cmd_completion(shell: &str) -> anyhow::Result<()> {
    use clap::CommandFactory;
    let shell = match shell {
        "bash" => clap_complete::Shell::Bash,
        "zsh" => clap_complete::Shell::Zsh,
        "fish" => clap_complete::Shell::Fish,
        "powershell" => clap_complete::Shell::PowerShell,
        "elvish" => clap_complete::Shell::Elvish,
        _ => anyhow::bail!("Unknown shell: {}. Supported: bash, zsh, fish, powershell, elvish", shell),
    };
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    clap_complete::generate(shell, &mut cmd, name, &mut std::io::stdout());
    Ok(())
}

// ─── Helpers ───────────────────────────────────────────────────

fn resolve_dir(dir: &Option<String>) -> anyhow::Result<String> {
    match dir {
        Some(d) => Ok(d.clone()),
        None => {
            // Try to use current directory as default, or error
            anyhow::bail!(
                "{}\n{}",
                format!("{} No database directory specified", "[ERROR]".red().bold()),
                "      Use --dir <path> or set PICKLEDB_DB environment variable".cyan()
            );
        }
    }
}

fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{} {}", bytes, UNITS[unit_idx])
    } else {
        format!("{:.2} {}", size, UNITS[unit_idx])
    }
}
