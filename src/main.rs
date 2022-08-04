use clap::Parser;
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
const CPP_GI: &str = "*.out
*.pch";
const CPP: &str = "int main(){

}";

const LUA_MF: &str = "";
const LUA: &str = "";

#[derive(Parser,)]
#[clap(about)]
struct TmpPrj {
   ft:   String,
   name: String,
}

fn main() {
   let tmplt = TmpPrj::parse();
   if tmplt.ft == "cpp".to_string() {
      let main_cpp = std::fs::create_dir(format!("./{}", &tmplt.name),);
   } else if tmplt.ft == "lua".to_string() {
   }
}
