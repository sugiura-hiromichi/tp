//MF->MakeFile, GI->.gitignore 'LauguageName'->Default Code
const CPP_MF: &str = "clngp_opt= -std=c++2a -Wall --pedantic-error

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

.PHONY : run clean re";
const CPP_GI: &str = "*.out";
const CPP: &str = "int main(){}";

const LUA_MF: &str = "";
const LUA: &str = "";

fn main() { println!("{CPP_MF}") }
