#include "SC_PlugIn.h"
#include <stdio.h>
#include "./bindings.h"

static InterfaceTable *ft;

struct MySynth : public Unit {
  struct UGenState* state;
};

extern "C" {
  void load(InterfaceTable *inTable);

  void MySynth_Ctor(MySynth* unit);
  void MySynth_next_a(MySynth* unit, int inNumSamples);
  void MySynth_Dtor(MySynth* unit);
  
};


void MySynth_Ctor(MySynth* unit) {
  SETCALC(MySynth_next_a);

  unit->state = new_state(SAMPLERATE);

  set_graph(unit->state);
    
  MySynth_next_a(unit, 1);
}


void MySynth_next_a(MySynth* unit, int inNumSamples) {
  
  UGenState* state = unit->state;

  process(state, unit->mInBuf, unit->mOutBuf, inNumSamples);

}

void MySynth_Dtor(MySynth* unit)
{

  state_free(unit->state);
  
}


PluginLoad(MySynth)
{
  ft = inTable;

  DefineDtorUnit(MySynth);
}
