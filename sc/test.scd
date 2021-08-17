s.reboot;

(
{
	var mouse1 = K2A.ar(MouseX.kr(100,1000,1));
	var mouse2 = K2A.ar(MouseY.kr(100,1000,1));
	var rust = RustSynth.ar(mouse1,mouse2).poll;
	//	var sine = SinOsc.ar(mouse+10).poll;
	rust
}.play
)

{ RustSynth.ar(K2A.ar(MouseX.kr(10,1000,1))).poll }.play

{ SinOsc.ar(K2A.ar(MouseX.kr(10,1000,1))).poll }.play

{ RustSynth.ar(SinOsc.ar(10).range(100,500)).poll }.play