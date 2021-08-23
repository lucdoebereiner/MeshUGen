s.reboot
s.volume = -6

~json = File.readAllString("/home/luc/Downloads/start_performance.json");
~jsonBuffer = Buffer.loadCollection(s, ~json.ascii, numChannels: 1);

// ~json = File.readAllString("/home/luc/Downloads/amazing.json");
// ~jsonBuffer = Buffer.loadCollection(s, ~json.ascii, numChannels: 1);

~json = File.readAllString("/home/luc/Downloads/fizzle.json");
~jsonBuffer = Buffer.loadCollection(s, ~json.ascii, numChannels: 1);


~json = File.readAllString("/home/luc/Downloads/start_small.json");
~jsonBuffer = Buffer.loadCollection(s, ~json.ascii, numChannels: 1);



(
{
	var edgesFac = 1;
	var edgesDel1 = 0.001, edgesDel2 = 0.0012, edgesDel3 = 0.00112, edgesDel4 = 0.0;
	var edgesFreq1 = 13000.0, edgesFreq2 = 6000.0, edgesFreq3 = 13000.0, edgesFreq4 = 13000.0;

	var rst = AudioMesh.ar(2, // channels
		~jsonBuffer.bufnum, // json bufnum
		MouseY.kr(0.1,3),
		MouseX.kr(0.0,1), // steto one for each channel, k-rate
		LFNoise1.kr(0.2).range(0,1), // steto one for each channel, k-rate
		edgesFac * 0.3,
		edgesFac * 0.23,
		edgesFac * 0.5,
		edgesFac * 0.7,
		edgesDel1,
		edgesDel2,
		edgesDel3,
		edgesDel4,
		edgesFreq1,
		edgesFreq2,
		edgesFreq3,
		edgesFreq4,
		LFNoise1.ar(2000)*0.1,
		LFNoise1.ar(2000)*0.1,
		//SinOsc.ar(500)*0.5, 
		//SinOsc.ar(112)*0.5
		
	);
	(HPF.ar(rst,20)*0.5);
}.play
)


(
{
	var edgesFac = 1;
	var edgesDel1 = 0.005, edgesDel2 = 0.001, edgesDel3 = 0.1, edgesDel4 = 0.02;
	var edgesFreq1 = 900.0, edgesFreq2 = 16000.0, edgesFreq3 = 1500.0, edgesFreq4 = 8000.0;

	var rst = AudioMesh.ar(2, // channels
		~jsonBuffer.bufnum, // json bufnum
		MouseY.kr(0.1,2),
		MouseX.kr(0.0,1), // steto one for each channel, k-rate
		LFNoise1.kr(0.2).range(0,1), // steto one for each channel, k-rate
		edgesFac * 0.5,
		edgesFac * 0.7,
		edgesFac,
		edgesFac * 1.5,
		edgesDel1,
		edgesDel2,
		edgesDel3,
		edgesDel4,
		edgesFreq1,
		edgesFreq2,
		edgesFreq3,
		edgesFreq4,
		LFNoise1.ar(1100)*0.1,
		LFNoise1.ar(100)*0.2,
		//SinOsc.ar(500)*0.5, 
		//SinOsc.ar(112)*0.5
	);
	(HPF.ar(rst,20)*0.5);
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


// inline double sc_round(double x, double quant) { return quant == 0. ? x : sc_floor(x / quant + .5) * quant; }

// inline float sc_trunc(float x, float quant) { return quant == 0. ? x : sc_floor(x / quant) * quant; }

// template <typename T> inline T sc_fold2(T a, T b) { return sc_fold(a, -b, b); }

// inline double sc_fold(double in, double lo, double hi) {
//     double x, c, range, range2;
//     x = in - lo;

//     // avoid the divide if possible
//     if (in >= hi) {
//         in = hi + hi - in;
//         if (in >= lo)
//             return in;
//     } else if (in < lo) {
//         in = lo + lo - in;
//         if (in < hi)
//             return in;
//     } else
//         return in;

//     if (hi == lo)
//         return lo;
//     // ok do the divide
//     range = hi - lo;
//     range2 = range + range;
//     c = x - range2 * sc_floor(x / range2);
//     if (c >= range)
//         c = range2 - c;
//     return c + lo;
// }


// 	*new1 { arg rate, which, array;
// 		var selector = UGen.methodSelectorForRate(rate);
// 		^this.crossfadeClass.perform(selector,
// 			Select.perform(selector, which.round(2), array),
// 			Select.perform(selector, which.trunc(2) + 1, array),
// 			(which * 2 - 1).fold2(1)
// 		);
// 	}


// 8.59.round(2)

// 8.59.trunc(2) + 1

// (4 * 2 - 1).fold2(1)

// 3.fold2(1)