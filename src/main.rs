// TODO: stop clap
#![feature(if_let_guard)]

mod templates;

use clap::Parser;
use mylibrary::sh_cmd;
use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::Write;
use templates::*;

#[derive(Parser,)]
#[clap(about)]
struct TmpPrj {
	/// filetype. Currently, rust, cpp, c, lua and journal are available
	ft:      String,
	/// project name
	name:    String,
	/// # available options:
	/// - `-pass=<value>` passing options to wrapped system
	options: Option<String,>,
	/// metadata
	/// - --version: show version information
	/// - --help: show help
	metas:   Option<String,>,
}

fn create_files(fstream: HashSet<FileBuf,>,) -> anyhow::Result<(),> {
	for fb in fstream {
		let mut f = fs::File::create(fb.name,)?;
		f.write(fb.context,)?;
	}
	Ok((),)
}

fn append_to_file(fstream: HashSet<FileBuf,>,) -> anyhow::Result<(),> {
	for fb in fstream {
		let mut f = fs::read_to_string(fb.name,)?;
		f.push_str(std::str::from_utf8(fb.context,)?,);
		fs::write(fb.name, f,)?;
	}
	Ok((),)
}

fn journal(prj_name: String,) -> anyhow::Result<(),> {
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
	create_files(HashSet::from([FileBuf { name, context: JOURNAL, },],),)
}

fn main() -> anyhow::Result<(),> {
	let tmplt = TmpPrj::parse();
	let prj_name = format!("./{}", &tmplt.name);
	let ft = tmplt.ft;

	match &ft[..] {
		"rs" => {
			let mut args = "new".to_string();
			if let Some(opt,) = tmplt.options {
				let mut val = opt.split('=',);
				val.next();
				args = args + " " + val.next().unwrap();
			}
			sh_cmd!("cargo", args.split_whitespace())?;
			append_to_file(HashSet::from([RS_GI, RS_TOML,],),)
		},
		"journal" => journal(prj_name.clone(),),
		_ => {
			fs::create_dir(prj_name.clone(),)?;

			// create list for each `ft`
			let mut fstream = if ft == "cpp" {
				Ok(HashSet::from([CPP, CPP_T, CPP_GI, CPP_H, CPP_MF,],),)
			} else if ft == "lua" {
				Ok(HashSet::from([LUA, LUA_T,],),)
			} else if ft == "c" {
				Ok(HashSet::from([C, C_T, CPP_GI, C_MF,],),)
			} else if ft == "swift" {
				Ok(HashSet::from([SWIFT, SWIFT_T,],),)
			} else {
				Err(io::Error::new(io::ErrorKind::NotFound, "unknown filetype",),)
			}?;
			fstream.insert(README,);

			sh_cmd!("cd", [prj_name])?;
			create_files(fstream,)
		},
	}
}
