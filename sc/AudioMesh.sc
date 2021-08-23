AudioMesh : MultiOutUGen {
	*ar { arg numChannels, bufnum=0 ... args;
		^this.multiNewList(['audio', numChannels, bufnum] ++ args);
	}

	init { arg ... theInputs;
		var argNumChannels = theInputs[0];
		inputs = theInputs;
		^this.initOutputs(argNumChannels, rate);
	}

}