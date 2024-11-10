pub mod compiler;
pub mod state;
pub mod mathcompiler;

use std::{env, fs::File, io::{Read, Write}, path::Path, process::{Command, Stdio}};

use compiler::compile;


#[tokio::main]
async fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() < 2 {
      println!("use this app by right-clicking any file in the explorer and selecting \"open with\", \"choose another app\", \"other apps\". Finally, scroll down to \"find another app on this pc\" and select this executable in the opened explorer window. If you don't want to repeat this entire process, double clicking the add.reg file in the file_server installation directory might help");
      return ();
   }
   let in_path = Path::new(&args[1]);
   if !args[1].ends_with(".qixtex"){
      println!("please poen a .qixtex file.");
      return ();
   }
   let argslen = args[1].len();
   let mut a1=args[1].clone();
   a1.truncate(argslen-".qixtex".len());
   println!("{}",args[1]);
   let pname = format!("{}.tex",a1);
let out_path = Path::new(&pname);
let mut oldin=String::new();
   loop{
      let mut input = String::new();
      File::open(in_path).unwrap().read_to_string(&mut input).unwrap();
      if oldin!=input{
         oldin=input.clone();
         let output=compile(input);
         File::create(out_path).unwrap().write_all(output.as_bytes()).unwrap();
         Command::new("pdflatex").arg(out_path.file_name().unwrap()).current_dir(out_path.parent().unwrap()).stdout(Stdio::inherit()).output().unwrap();
         println!("compiled!");
      }else{
         std::thread::sleep(std::time::Duration::from_secs(1));
      }
   }
}
