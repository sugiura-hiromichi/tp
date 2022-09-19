mod templates;

use {
   clap::Parser,
   std::{
      fs,
      io::{self, Write},
      os::unix::process::CommandExt,
   },
   templates::*,
};

#[derive(Parser,)]
#[clap(about)]
struct TmpPrj {
   ///filetype. Currently, cpp, lua and journal are available
   ft:     String,
   ///project name
   name:   String,
   ///NOTE This option is only valid for journal. Select w: Weekly, m: Monthly, y: Year
   #[clap(value_parser)]
   period: char,
}

fn create_files(fstream: Vec<FileBuf,>, prj_name: String,) -> io::Result<(),> {
   for fb in fstream {
      let mut f = fs::File::create(format!("{prj_name}/{}", fb.name),)?;
      f.write(fb.context,)?;
   }
   Ok((),)
}

fn journal(prj_name: String, period: char,) -> io::Result<(),> {
   let name = match period {
      'w' => Ok(prj_name.as_str(),),
      'm' => {
         fs::create_dir(prj_name,)?;
         Ok("MONTHLYLOG.md",)
      },
      'y' => {
         fs::create_dir(prj_name,)?;
         Ok("YEARLOG.md",)
      },
      _ => Err(io::Error::new(
         io::ErrorKind::NotFound,
         "unknown option for period. period must be 'w' or 'm' or 'y'",
      ),),
   }?;
   let fstream = vec![FileBuf { name, context: JOURNAL, }];
   create_files(fstream, "./".to_string(),)
}

fn main() -> io::Result<(),> {
   let tmplt = TmpPrj::parse();
   let prj_name = format!("./{}", &tmplt.name);
   let ft = tmplt.ft;
   if "journal".to_string() == ft {
      return journal(prj_name.clone(), tmplt.period,);
   }
   fs::create_dir(prj_name.clone(),)?;

   let fstream = if ft == "cpp".to_string() {
      Ok(vec![CPP, CPP_GI, CPP_H, CPP_MF],)
   } else if ft == "lua".to_string() {
      Ok(vec![LUA, LUA_MF],)
   } else {
      Err(io::Error::new(io::ErrorKind::NotFound, "unknown filetype",),)
   }?;
   create_files(fstream, prj_name.clone(),)?;

   std::process::Command::new("z",).arg(prj_name,).exec();
   Ok((),)
}
