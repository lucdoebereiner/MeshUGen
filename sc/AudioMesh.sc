AudioMesh : MultiOutUGen {
	*ar { arg numChannels, bufnum=0 ... args;
		^this.multiNewList(['audio', numChannels, bufnum] ++ args);
	}

	init { arg argNumChannels ... theInputs;
		inputs = theInputs;
		^this.initOutputs(argNumChannels, rate);
	}

}