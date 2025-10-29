use crate::blockchain::Blockchain;

use clap::{Command, arg};
use std::process::exit;
use std::{mem, fs};
use anyhow::Result;

pub struct Cli{
    bc: Blockchain,
}

impl Cli{

    pub fn new() -> Result<Cli>{
        Ok(
            Cli{
                bc: Blockchain::new()?,
            }
        )
    }

    pub fn run(&mut self) -> Result<()>{
        let matches =Command::new("Blockchain-rust-demo")
            .version("0.1")
            .author("pedro@mgmail.com")
            .about("blockchain in rust")
            .subcommand(Command::new("printchain")
            .about("prrint all the blocks"))
            .subcommand(Command::new("clear")
            .about("clear the blockchain"))
            .subcommand(Command::new("addblock")
                .about("add a block in the blockchain")
                .arg(arg!(<DATA>"'The block data'")),
        )
        .get_matches();

        if let Some(ref matches) = matches.subcommand_matches("addblock"){
            if let Some(c) = matches.get_one::<String>("DATA"){
                self.addblock(String::from(c))?;
            } else{
                println!("Failure in the command");
                exit(1)
            }
        }

        if let Some(_) = matches.subcommand_matches("printchain"){
            self.cmd_print_chain()?;
        }

        if let Some(_) = matches.subcommand_matches("clear"){
            self.clear_blockchain()?;
        }

        Ok(())
    }

    pub fn cmd_print_chain(&self) -> Result<()>{
        for b in self.bc.iter(){
            println!("{:?}", b);
        }

        Ok(())
    }

    pub fn addblock(&mut self, data: String) -> Result<()>{
        println!("Adding block with data {:?}", data);
        self.bc.add_block(data)?;
        Ok(())
    }

    pub fn clear_blockchain(&mut self) -> Result<()> {
        println!("Clearing blockchain...");

        // 1️⃣ Garante que tudo pendente foi escrito
        self.bc.db.flush()?;

        // 2️⃣ Substitui o DB atual por um temporário vazio
        let db_default = sled::open("/tmp/empty_sled")?;
        let old_db = mem::replace(&mut self.bc.db, db_default);

        // 3️⃣ Drop do antigo e espera curta pra liberar file locks
        drop(old_db);
        std::thread::sleep(std::time::Duration::from_millis(200));

        // 4️⃣ Remove os dados antigos
        let db_path = "data/blocks";
        if std::path::Path::new(db_path).exists() {
            fs::remove_dir_all(db_path)?;
        }

        println!("✅ Blockchain cleared!");
        std::process::exit(0);
    }
}