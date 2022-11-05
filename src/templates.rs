//! Contain const variables

pub struct FileBuf<'a,> {
   pub(crate) name:    &'a str,
   pub(crate) context: &'a [u8],
}

//MF->Makefile, GI->.gitignore 'LauguageName'->Default Code
pub const CPP_MF: FileBuf = FileBuf {
   name:    "Makefile",
   context: b"clngp_opt= -std=c++2a -Wall --pedantic-error

r : a.out
\t./a.out

a.out : main.cpp all.h all.h.pch
\tclang++ $(clngp_opt) -include all.h $< -o $@

all.h.pch : all.h
\tclang++ $(clngp_opt) -x c++-header -o $@ $<

clean :
\trm -f ./a.out
\trm -f ./all.h.pch

.PHONY : r clean",
};

pub const CPP_H: FileBuf = FileBuf {
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

pub const CPP_GI: FileBuf = FileBuf {
   name:    ".gitignore",
   context: b"*.out
*.pch",
};

pub const CPP: FileBuf = FileBuf {
   name:    "main.cpp",
   context: b"#include \"all.h\"
using namespace std;
int main(){

}",
};

pub const C_MF: FileBuf = FileBuf {
   name:    "Makefile",
   context: b"r : a.out
\t./a.out

a.out : main.c
\tclang $<

clean :
\trm -f ./a.out

.PHONY : r clean",
};

pub const C: FileBuf = FileBuf {
   name:    "main.c",
   context: b"#include <stdio.h>
int main(){

}",
};

pub const LUA_MF: FileBuf = FileBuf {
   name:    "Makefile",
   context: b"r : main.lua
\tlua main.lua",
};

pub const LUA: FileBuf = FileBuf { name: "main.lua", context: b"", };

pub const JOURNAL: &[u8] = b" # What I want to carry out during 2022?

### Social Require


### Progress visualize


### Creation


### Odd-jobs

---

# Result

";
