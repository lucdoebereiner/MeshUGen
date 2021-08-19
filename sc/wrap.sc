AudioMesh : MultiOutUGen {
	*ar { arg buf, ... args;
		^this.multiNewList(['audio', buf] ++ args);
	}

	init {arg ... theInputs;
		inputs = theInputs;
		^this.initOutputs(2, 'audio');
	}	

}