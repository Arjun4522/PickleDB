use std::env;
use std::fs;

use pickledb_core::{
    traits::{Client, Engine},
    types::{InsertTuple, RecordId, SearchToken},
};
use pickledb_crypto::client::PickleClient;
use pickledb_engine::engine::PickleEngine;

fn print_usage() {
    eprintln!("Usage: pickledb-cli <dir> <command> [args...]");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  init                    Initialize a new database directory");
    eprintln!("  insert <id> <data>      Insert a record with the given id and data");
    eprintln!("  get <id>                Get and decrypt a record by id");
    eprintln!("  search <token_hex>      Search for records by search token (hex)");
    eprintln!("  delete <id>             Delete a record by id");
    eprintln!("  sync                    Flush all pending writes");
    eprintln!("  checkpoint              Create a WAL checkpoint");
    eprintln!("  compact                 Compact storage");
    eprintln!("  stats                   Show database statistics");
}

fn get_master_key() -> anyhow::Result<Vec<u8>> {
    let key_str = env::var("PICKLEDB_KEY")
        .map_err(|_| anyhow::anyhow!("PICKLEDB_KEY environment variable not set (must be 32 bytes)"))?;
    let key = key_str.as_bytes().to_vec();
    anyhow::ensure!(key.len() == 32, "PICKLEDB_KEY must be exactly 32 bytes");
    Ok(key)
}

fn cmd_init(dir: &str) -> anyhow::Result<()> {
    fs::create_dir_all(dir)?;
    let mut engine = PickleEngine::open(dir)?;
    engine.sync()?;
    println!("Initialized PickleDB at {}", dir);
    Ok(())
}

fn cmd_insert(dir: &str, id: u64, data: &str) -> anyhow::Result<()> {
    let key = get_master_key()?;
    let client = PickleClient::new(&key)?;
    let mut engine = PickleEngine::open(dir)?;

    let payload = client.encrypt(RecordId(id), data.as_bytes())?;
    let tuple = InsertTuple {
        record_id: RecordId(id),
        payload,
        search_tokens: vec![],
    };
    engine.insert(tuple)?;
    println!("Inserted record {} into {}", id, dir);
    Ok(())
}

fn cmd_get(dir: &str, id: u64) -> anyhow::Result<()> {
    let key = get_master_key()?;
    let client = PickleClient::new(&key)?;
    let engine = PickleEngine::open(dir)?;

    match engine.get(RecordId(id)) {
        Ok(payload) => {
            match client.decrypt(RecordId(id), &payload) {
                Ok(plaintext) => {
                    let s = String::from_utf8_lossy(&plaintext);
                    println!("Record {}: {}", id, s);
                }
                Err(e) => {
                    println!("Record {}: failed to decrypt: {}", id, e);
                }
            }
        }
        Err(e) => {
            println!("Record {} not found: {}", id, e);
        }
    }
    Ok(())
}

fn cmd_search(dir: &str, token_hex: &str) -> anyhow::Result<()> {
    let bytes = hex::decode(token_hex)
        .map_err(|_| anyhow::anyhow!("invalid hex token"))?;
    if bytes.len() != 32 {
        anyhow::bail!("search token must be 32 bytes (64 hex chars)");
    }
    let mut token_bytes = [0u8; 32];
    token_bytes.copy_from_slice(&bytes);
    let token = SearchToken(token_bytes);

    let engine = PickleEngine::open(dir)?;
    match engine.search(&token) {
        Ok(records) => {
            println!("Found {} matching records:", records.len());
            for id in records {
                println!("  RecordId({})", id.0);
            }
        }
        Err(_) => {
            println!("No matching records found");
        }
    }
    Ok(())
}

fn cmd_delete(dir: &str, id: u64) -> anyhow::Result<()> {
    let mut engine = PickleEngine::open(dir)?;
    engine.delete(RecordId(id))?;
    println!("Deleted record {}", id);
    Ok(())
}

fn cmd_sync(dir: &str) -> anyhow::Result<()> {
    let mut engine = PickleEngine::open(dir)?;
    engine.sync()?;
    println!("Synced {}", dir);
    Ok(())
}

fn cmd_checkpoint(dir: &str) -> anyhow::Result<()> {
    let mut engine = PickleEngine::open(dir)?;
    engine.checkpoint()?;
    println!("Checkpoint created at {}", dir);
    Ok(())
}

fn cmd_compact(dir: &str) -> anyhow::Result<()> {
    let mut engine = PickleEngine::open(dir)?;
    engine.compact()?;
    println!("Storage compacted at {}", dir);
    Ok(())
}

fn cmd_stats(dir: &str) -> anyhow::Result<()> {
    let _engine = PickleEngine::open(dir)?;
    println!("PickleDB stats for {}", dir);
    println!("  Engine type: Encrypted embedded database");
    println!("  Status: Open");
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_usage();
        std::process::exit(1);
    }

    let dir = &args[1];
    let command = &args[2];

    match command.as_str() {
        "init" => cmd_init(dir)?,
        "insert" => {
            if args.len() < 5 {
                anyhow::bail!("Usage: pickledb-cli <dir> insert <id> <data>");
            }
            let id: u64 = args[3].parse()?;
            cmd_insert(dir, id, &args[4])?;
        }
        "get" => {
            if args.len() < 4 {
                anyhow::bail!("Usage: pickledb-cli <dir> get <id>");
            }
            let id: u64 = args[3].parse()?;
            cmd_get(dir, id)?;
        }
        "search" => {
            if args.len() < 4 {
                anyhow::bail!("Usage: pickledb-cli <dir> search <token_hex>");
            }
            cmd_search(dir, &args[3])?;
        }
        "delete" => {
            if args.len() < 4 {
                anyhow::bail!("Usage: pickledb-cli <dir> delete <id>");
            }
            let id: u64 = args[3].parse()?;
            cmd_delete(dir, id)?;
        }
        "sync" => cmd_sync(dir)?,
        "checkpoint" => cmd_checkpoint(dir)?,
        "compact" => cmd_compact(dir)?,
        "stats" => cmd_stats(dir)?,
        _ => {
            print_usage();
            anyhow::bail!("Unknown command: {}", command);
        }
    }

    Ok(())
}
