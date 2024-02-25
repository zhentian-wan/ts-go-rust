use anyhow::{Result, anyhow, Context};
use std::path::PathBuf;
use crate::opts::Opts;

#[derive(Debug)]
pub struct Config {
  pub operation: Operation,
  pub pwd: PathBuf,
  pub config: PathBuf,
}

impl TryFrom<Opts> for Config {
  type Error = anyhow::Error;

  fn try_from(value: Opts) -> Result<Self> {
    // when you impl TryFrom, you can use try_into
    let operation = value.args.try_into()?;
    let config = get_config(value.config)?;
    let pwd = get_pwd(value.pwd)?;

    return Ok(Config {
      operation,
      config,
      pwd,
    });
  }
}

#[derive(Debug)]
pub enum Operation {
  Print(Option<String>),
  Add(String, String),
  Remove(String),
}

impl TryFrom<Vec<String>> for Operation {
  type Error = anyhow::Error;

  fn try_from(value: Vec<String>) -> Result<Self> {
    let mut value = value;
    if value.len() == 0 {
      return Ok(Operation::Print(None));
    }

    let term = value.get(0).expect("expect to exist");
    if term == "add" {
      if value.len() != 3 {
        return Err(anyhow!("opearation add expected 2 arguements but got {}", value.len() - 1));
      }

      let mut drain = value.drain(1..=2);
      return Ok(
        Operation::Add(drain.next().expect("to exist"), drain.next().expect("to exist"))
      )
    }

    if term == "Remove" {
      if value.len() != 2 {
        return Err(anyhow!("opearation remove expected 1 arguements but got {}", value.len() - 1));
      }

      let arg = value.pop().expect("to exist");
      return Ok(
        Operation::Remove(arg)
      );
    }

    if value.len() > 1 {
      return Err(anyhow!("operation pritn expects 0 or 1 arguments, but got {}", value.len()));
    }

    let arg = value.pop().expect("to exist");
    return Ok(Operation::Print(Some(arg)));
  }
}

fn get_config(config: Option<PathBuf>) -> Result<PathBuf> {
  if let Some(v) = config {
    return Ok(v);
  }

  let loc = std::env::var("XDG_CONFIG_HOME").context("unable to get XDG_CONFIG_HOME")?;
  let mut loc = PathBuf::from(loc);

  loc.push("projector");
  loc.push("projector.json");

  return Ok(loc);
}

fn get_pwd(pwd: Option<PathBuf>) -> Result<PathBuf> {
  if let Some(pwd) = pwd {
    return Ok(pwd);
  }

  return Ok(std::env::current_dir().context("errored getting current_dir")?)
}

