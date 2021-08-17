MySynth : MultiOutUGen {
	*ar { arg f1, f2;
		^this.multiNewList(['audio', f1,f2,f3]);
	}

	init { arg ... theInputs;
		inputs = theInputs;
		^this.initOutputs(2, 'audio');
	}	

}