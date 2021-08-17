#include "SC_PlugIn.h"
#include <stdio.h>
#include "./bindings.h"

static InterfaceTable *ft;

struct RustSynth : public Unit {
  struct UGenState* state;
};

extern "C" {
  void load(InterfaceTable *inTable);

  void RustSynth_Ctor(RustSynth* unit);
  void RustSynth_next_a(RustSynth* unit, int inNumSamples);
  void RustSynth_Dtor(RustSynth* unit);
  
};


void RustSynth_Ctor(RustSynth* unit) {
  SETCALC(RustSynth_next_a);

  unit->state = new_state(SAMPLERATE);

  set_graph(unit->state);
    
  RustSynth_next_a(unit, 1);
}


void RustSynth_next_a(RustSynth* unit, int inNumSamples) {
  
  UGenState* state = unit->state;

  process(state, unit->mInBuf, unit->mOutBuf, inNumSamples);

}

void RustSynth_Dtor(RustSynth* unit)
{

  state_free(unit->state);
  
}


PluginLoad(RustSynth)
{
  ft = inTable;

  DefineDtorUnit(RustSynth);
}


