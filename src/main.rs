use {
   clap::Parser,
   std::{
      fs,
      io::{self, Write},
   },
};

//MF->MakeFile, GI->.gitignore 'LauguageName'->Default Code
const CPP_MF: FileBuf = FileBuf {
   name:    "MakeFile",
   context: b"clngp_opt= -std=c++2a -Wall --pedantic-error

run : a.out
\t./a.out

a.out : main.cpp all.h all.h.pch
\tclang++ $(clngp_opt) -include all.h $< -o $@

all.h.pch : all.h
\tclang++ $(clngp_opt) -x c++-header -o $@ $<

clean :
\trm -f ./a.out
\trm -f ./all.h.pch

re : clean run

.PHONY : run clean re",
};

const CPP_H: FileBuf = FileBuf {
   name:    "all.h",
   context: b"#include <cstddef>
#include <limits>
#include <climits>
#include <cfloat>
#include <cstdint>
#include <cstdlib>
#include <new>
#include <typeinfo>
#include <exception>
#include <initializer_list>
#include <stdexcept>
#include <cassert>
#include <cerrno>
#include <system_error>
#include <string>

#if __has_include(<string_view>)
#   include <string_view>
#endif

#include <array>
#include <deque>
#include <forward_list>
#include <list>
#include <vector>
#include <map>
#include <set>
#include <unordered_map>
#include <unordered_set>
#include <queue>
#include <stack>
#include <iterator>
#include <algorithm>
#include <cfenv>
#include <random>
#include <numeric>
#include <cmath>
#include <iosfwd>
#include <iostream>
#include <ios>
#include <streambuf>
#include <istream>
#include <ostream>
#include <iomanip>
#include <sstream>
#include <fstream>

#if __has_include(<filesystem>)
#   include <filesystem>
#endif

#include <cstdio>
#include <cinttypes>


#include <regex>
#include <atomic>
#include <thread>
#include <mutex>
#include <shared_mutex>
#include <condition_variable>
#include <future>

using namespace std::literals ;",
};

const CPP_GI: FileBuf = FileBuf {
   name:    ".gitignore",
   context: b"*.out
*.pch",
};

const CPP: FileBuf = FileBuf {
   name:    "main.cpp",
   context: b"#include \"all.h\"
using namespace std;
int main(){}",
};

const LUA_MF: FileBuf = FileBuf {
   name:    "MakeFile",
   context: b"run : main.lua
\tlua main.lua",
};

const LUA: FileBuf = FileBuf { name: "main.lua", context: b"", };

#[derive(Parser,)]
#[clap(about)]
struct TmpPrj {
   ft:   String,
   name: String,
}

struct FileBuf<'a,> {
   name:    &'a str,
   context: &'a [u8],
}

fn main() -> io::Result<(),> {
   let tmplt = TmpPrj::parse();
   let prj_name = format!("./{}", &tmplt.name);
   let ft = tmplt.ft;
   std::fs::create_dir(prj_name.clone(),)?;

   let fstream = if ft == "cpp".to_string() {
      Ok(vec![CPP, CPP_GI, CPP_H, CPP_MF],)
   } else if ft == "lua".to_string() {
      Ok(vec![LUA, LUA_MF],)
   } else {
      Err(io::Error::new(io::ErrorKind::NotFound, "unknown filetype",),)
   }?;

   for fb in fstream {
      let mut f = fs::File::create(format!("{prj_name}/{}", fb.name),)?;
      f.write(fb.context,)?;
   }
   Ok((),)
}
