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

  float fbufnum = ZIN0(0);
  uint32 bufnum = (int)fbufnum;
  World* world = unit->mWorld;
  if (bufnum >= world->mNumSndBufs)
    bufnum = 0;
  const SndBuf* buf = world->mSndBufs + bufnum;
  ACQUIRE_SNDBUF_SHARED(buf);
  
  const float* bufData __attribute__((__unused__)) = buf->data;
  uint32 bufChannels __attribute__((__unused__)) = buf->channels;
  uint32 bufSamples __attribute__((__unused__)) = buf->samples;
  uint32 bufFrames = buf->frames;
  int mask __attribute__((__unused__)) = buf->mask;
  int guardFrame __attribute__((__unused__)) = bufFrames - 2;
  
  set_graph(unit->state, bufData, bufFrames);

  RELEASE_SNDBUF_SHARED(buf);
  
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


