AudioMesh : MultiOutUGen {
	*ar { arg buf, ... args;
		^this.multiNewList(['audio', buf] ++ args);
	}

	init { arg argNumChannels ... theInputs;
		inputs = theInputs;
		^this.initOutputs(argNumChannels, rate);
	}

}