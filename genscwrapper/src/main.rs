use clap::Clap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn gen_sc(name: String, outputs: usize, args: Vec<String>) -> String {
    let arg_str = args.join(",");
    format!(
        r#"{name} : MultiOutUGen {{
	*ar {{ arg f1, f2;
		^this.multiNewList(['audio', {args}]);
	}}

	init {{ arg ... theInputs;
		inputs = theInputs;
		^this.initOutputs({outputs}, 'audio');
	}}	

}}"#,
        name = name,
        outputs = outputs,
        args = arg_str
    )
}

fn gen_cpp(name: String) -> String {
    format!(
        r#"#include "SC_PlugIn.h"
#include <stdio.h>
#include "./bindings.h"

static InterfaceTable *ft;

struct {name} : public Unit {{
  struct UGenState* state;
}};

extern "C" {{
  void load(InterfaceTable *inTable);

  void {name}_Ctor({name}* unit);
  void {name}_next_a({name}* unit, int inNumSamples);
  void {name}_Dtor({name}* unit);
  
}};


void {name}_Ctor({name}* unit) {{
  SETCALC({name}_next_a);

  unit->state = new_state(SAMPLERATE);

  set_graph(unit->state);
    
  {name}_next_a(unit, 1);
}}


void {name}_next_a({name}* unit, int inNumSamples) {{
  
  UGenState* state = unit->state;

  process(state, unit->mInBuf, unit->mOutBuf, inNumSamples);

}}

void {name}_Dtor({name}* unit)
{{

  state_free(unit->state);
  
}}


PluginLoad({name})
{{
  ft = inTable;

  DefineDtorUnit({name});
}}
"#,
        name = name
    )
}

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Luc Döbereiner <luc.doebereiner@gmail.com>")]
struct Opts {
    #[clap(short, long)]
    name: String,
    #[clap(short, long)]
    outputs: usize,
    #[clap(short, long)]
    arguments: Vec<String>,
    #[clap(short, long, default_value = "../sc")]
    dir: String,
}

fn main() -> std::io::Result<()> {
    let opts: Opts = Opts::parse();
    let cpp = gen_cpp(opts.name.to_string());
    let sc = gen_sc(opts.name.to_string(), opts.outputs, opts.arguments);
    let cpp_path = Path::new(&opts.dir)
        .join(opts.name.to_string())
        .with_extension("cpp");

    let sc_path = Path::new(&opts.dir)
        .join(opts.name.to_string())
        .with_extension("sc");

    let mut output_cpp = File::create(cpp_path.to_str().unwrap())?;
    let mut output_sc = File::create(sc_path.to_str().unwrap())?;
    // let line = "hello";
    write!(output_cpp, "{}", cpp)?;
    write!(output_sc, "{}", sc)?;

    Ok(())
}
