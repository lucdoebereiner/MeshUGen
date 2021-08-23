#include "SC_PlugIn.h"
#include <stdio.h>
#include "./bindings.h"

static InterfaceTable *ft;

struct AudioMesh : public Unit {
  struct UGenState* state;
};

extern "C" {
  void load(InterfaceTable *inTable);

  void AudioMesh_Ctor(AudioMesh* unit);
  void AudioMesh_next_a(AudioMesh* unit, int inNumSamples);
  void AudioMesh_Dtor(AudioMesh* unit);
  
};


void AudioMesh_Ctor(AudioMesh* unit) {
  SETCALC(AudioMesh_next_a);

  unit->state = new_state(SAMPLERATE);

  float fbufnum = IN0(1);
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


  float n_outputs = IN0(0);
  set_graph(unit->state, bufData, bufFrames, (uint32) n_outputs);

  RELEASE_SNDBUF_SHARED(buf);
  
  AudioMesh_next_a(unit, 1);
}


void AudioMesh_next_a(AudioMesh* unit, int inNumSamples) {
  
  UGenState* state = unit->state;

  process(state, unit->mInBuf, unit->mOutBuf, inNumSamples);

}

void AudioMesh_Dtor(AudioMesh* unit)
{

  state_free(unit->state);
  
}


PluginLoad(AudioMesh)
{
  ft = inTable;

  DefineDtorUnit(AudioMesh);
}


