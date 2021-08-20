s.reboot

// ~json = File.readAllSignal("/home/luc/Downloads/amazing.json");
// ~jsonBuffer = Buffer.loadCollection(s, ~json, numChannels: 1);

// ~jsonBuffer.plot


~json = File.readAllString("/home/luc/Downloads/start_performance.json");
~jsonBuffer = Buffer.loadCollection(s, ~json.ascii, numChannels: 1);


(
{
	var rst = AudioMesh.ar(2,~jsonBuffer.bufnum,
		K2A.ar(MouseX.kr(0.001,2,1)),
		SinOsc.ar(MouseX.kr(0.01,1000,1))*0.1,
		SinOsc.ar(MouseY.kr(0.01,1000,1))*0.1);
	(HPF.ar(rst,20)*0.5);
}.play
)


~json = File.readAllString("/home/luc/Downloads/fizzle.json");
~jsonBuffer = Buffer.loadCollection(s, ~json.ascii, numChannels: 1);


(
{
	var rst = AudioMesh.ar(2,~jsonBuffer.bufnum,
		K2A.ar(MouseX.kr(0.01,10,1)),
		SinOsc.ar(MouseX.kr(0.01,1000,1))*0.1,
		SinOsc.ar(MouseY.kr(0.01,1000,1))*0.2);
	(HPF.ar(rst,20)*0.4);
}.play
)



~json = File.readAllString("/home/luc/Downloads/start_small.json");
~jsonBuffer = Buffer.loadCollection(s, ~json.ascii, numChannels: 1);


(
{
	var rst = AudioMesh.ar(2,~jsonBuffer.bufnum,
		K2A.ar(MouseX.kr(0.01,10,1)),
		SinOsc.ar(MouseX.kr(0.01,1000,1))*0.2,
		SinOsc.ar(MouseY.kr(0.01,1000,1))*0.2);
	(HPF.ar(rst,20)*0.6);
}.play
)

~json = File.readAllString("/home/luc/Downloads/sintest.json");
~jsonBuffer = Buffer.loadCollection(s, ~json.ascii, numChannels: 1);


(
{
	var rst = AudioMesh.ar(2,~jsonBuffer.bufnum,
		K2A.ar(MouseX.kr(0.01,2,1)),
		DC.ar(0),
		DC.ar(0)) * 0.1;
	rst;
}.play
)