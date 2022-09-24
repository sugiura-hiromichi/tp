#![allow(unused)]

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
   ft:   String,
   ///project name
   name: String,
}

fn create_files(fstream: Vec<FileBuf,>, prj_name: String,) -> io::Result<(),> {
   for fb in fstream {
      let mut f = fs::File::create(format!("{prj_name}/{}", fb.name),)?;
      f.write(fb.context,)?;
   }
   Ok((),)
}

fn journal(prj_name: String,) -> io::Result<(),> {
   //remove first "./" of prj_name
   let name = match &prj_name[2..].parse::<i32>() {
      Ok(m_y,) => match m_y {
         m @ 1..=12 => {
            let dir = match m {
               1 => "1_January",
               2 => "2_February",
               3 => "3_March",
               4 => "4_April",
               5 => "5_May",
               6 => "6_June",
               7 => "7_July",
               8 => "8_August",
               9 => "9_September",
               10 => "10_October",
               11 => "11_November",
               12 => "12_December",
               _ => panic!("error happen in journal()"),
            };
            fs::create_dir(dir,)?;
            dir.to_owned() + "/MONTHLYLOG.md"
         },
         2022..=i32::MAX => {
            fs::create_dir(&prj_name,)?;
            prj_name.to_owned() + "/YEARLOG.md"
         },
         _ => "journal_template".to_string(),
      },
      Err(_,) => prj_name,
   };
   let name = &format!("{name}.md");
   let fstream = vec![FileBuf { name, context: JOURNAL, }];
   create_files(fstream, "./".to_string(),)
}

fn main() -> io::Result<(),> {
   let tmplt = TmpPrj::parse();
   let prj_name = format!("./{}", &tmplt.name);
   let ft = tmplt.ft;
   if "journal".to_string() == ft {
      return journal(prj_name.clone(),);
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

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn parse_str_to_num() {
      assert_eq!("1".to_string().parse::<i32>(), Ok(1));
   }
}
