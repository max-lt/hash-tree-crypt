mod file;
mod tree;
mod args;

use std::{fs::OpenOptions, io::BufWriter, path::PathBuf};
use log::{info, error};

use crate::{args::Args, file::encrypt_stream};

fn main() {
  let input_isnt_stdin = atty::isnt(atty::Stream::Stdin);

  let matches = args::cli(input_isnt_stdin).get_matches();

  // Print version and exit
  if matches.get_flag(Args::Version.into()) {
    println!("hash-tree-crypt v{}", std::env!("CARGO_PKG_VERSION"));
    return;
  }

  let log_debug = matches.get_flag(Args::Debug.into());

  // Id debug flag is set, we set the log level to debug
  if log_debug {
    std::env::set_var("RUST_LOG", "debug");
  }
  // Else if no log level is set, we set it to info
  else if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "info");
  }

  env_logger::init();
  
  let source = matches.get_one::<PathBuf>(Args::Input.into());
  if source.is_none() && !input_isnt_stdin {
    error!("Error: no input file");
    return;
  }

  // Checking input file
  let metadata = match source {
    None => None,
    Some(source) => {
      // Get file metadata
      let metadata = match std::fs::metadata(&source) {
        Ok(m) => m,
        Err(reason) => {
          error!("Error while checking {:?}: {}", source, reason);
          return;
        }
      };

      // Checking if file
      if !metadata.is_file() {
        error!("Error: {:?} is not a file", source);
        return;
      }

      // Prevent reading big file in debug mode
      if log::log_enabled!(log::Level::Debug) {
        if metadata.len() > 1024*1024 {
          error!("You don't want to print debug logs for a such large file{}", if log_debug { "" } else { " (RUST_LOG=debug)" });
          return;
        }
      }

      Some(metadata)
    }
  };

  // Initializing tree
  let mut tree = {
    eprint!("Enter encryption password: ");
    let password = rpassword::read_password().unwrap();

    eprint!("Verifying encryption password: ");
    if password != rpassword::read_password().unwrap() {
      error!("Password verification failed.");
      return;
    }

    // Seed is the hash of the password
    let seed = blake3::hash(password.as_bytes());

    // Create our tree
    tree::HashTree::create(32, 0, seed)
  };

  // Check if file size < pad size
  match metadata {
    Some(metadata) => {
      let max_file_size = tree.last_byte_index();
      
      if metadata.len() > max_file_size as u64 {
        error!("Error: {:?} is too big ({} > {})", source, metadata.len(), max_file_size);
        return;
      }
    },
    None => {
      info!("Max stream size: {} bytes", tree.last_byte_index());
    }
  }

  match source {
    Some(source) => info!("Input file:  {}", source.to_str().unwrap()),
    None => info!("Input from stdin")
  }

  // Create writer
  let writer: BufWriter<Box<dyn std::io::Write>> = {

    let dest = matches.get_one::<PathBuf>(Args::Output.into());

    // Output is not a tty
    let output_stream = atty::isnt(atty::Stream::Stdout);
    
    let user_set_output = dest.is_some();

    // If output is not a tty and no output file is set, we output to stdout
    match output_stream && !user_set_output {
      // Output is stdout
      true => {
        info!("Output to stdout");

        BufWriter::new(Box::new(std::io::stdout()))
      },
      // Output is a file
      false => {
        let dest = match dest {
          Some(p) => p.to_owned(),
          None => match source {
              // If source is stdin, we output to output.htcrypt
              None => PathBuf::from("output.htcrypt"),
              Some(source) => {
                // Take source file name and add .htcrypt extension
                let mut p = source.clone();
                p.set_file_name(p.file_name().unwrap().to_str().unwrap().to_string() + ".htcrypt");
                p.to_owned()
              },
            }
        };

        // If dest is auto generated, we ensure we don't overwrite an existing file
        let dest = match !user_set_output && dest.exists() {
          false => dest,
          true => {
            let mut dest = dest;
            let time = chrono::Local::now().timestamp_millis().to_string();
            dest.set_extension(time + ".htcrypt" );
            dest
          },
        };

        info!("Output file: {}", dest.to_str().unwrap());

        match OpenOptions::new().read(true).write(true).create(true).open(&dest) {
            Ok(file) => BufWriter::new(Box::new(file)),
            Err(reason) => {
              error!("Error while opening {:?}: {}", dest, reason);
              return;
            }
          }
        }
      }
  };

  // Open input file
  let reader: std::io::BufReader<Box<dyn std::io::Read>> = match source {
      Some(source) => match OpenOptions::new().read(true).open(&source) {
      Ok(file) => std::io::BufReader::new(Box::new(file)),
      Err(reason) => {
        error!("Error while opening {:?}: {}", source, reason);
        return;
      }
    },
    None => std::io::BufReader::new(Box::new(std::io::stdin()))
  };

  info!("Tree: last leaf index: {}, max file size: {}", tree.last_leaf_index(), tree.last_byte_index());

  match encrypt_stream(reader, writer, &mut tree) {
    Ok(_) => (),
    Err(reason) => {
      error!("Error while reading {:?}: {}", source, reason);
      return;
    }
  }

  info!("File encrypted successfully");
  info!("To reverse the process, run the same command on the encrypted file");
}
