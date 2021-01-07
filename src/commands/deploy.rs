use std::error::Error;

use super::command::GuanCommand;
use crate::pipeline::Pipeline;

pub struct DeployArgs {
  pub pipeline_file_path: String,
  pub workdir: String,
}

pub struct DeployCommand {
  args: DeployArgs,
}

impl DeployCommand {
  pub fn new(args: DeployArgs) -> DeployCommand {
    DeployCommand { args }
  }
}

impl GuanCommand for DeployCommand {
  fn execute(&self) -> Result<(), Box<dyn Error>> {
    let pipeline = Pipeline::load_from_file(&self.args.pipeline_file_path)?;

    for stage in pipeline.stages.iter() {
      println!("Running {}", stage.name);
      match stage.run(&self.args.workdir) {
        Err(stderr) => {
          println!("Something went wrong. Stderr:\n\n{}", stderr);
          break;
        }
        _ => {
          println!("Done!");
        }
      }
    }

    Ok(())
  }
}
