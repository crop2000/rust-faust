declare name 		"saw";
declare version 	"1.0";
declare author 		"Grame";
declare license 	"BSD";
declare copyright 	"(c)GRAME 2009";

//-----------------------------------------------
// 			Saw Oscillator
//-----------------------------------------------

import("stdfaust.lib");

vol = hslider("volume [unit:dB]", 0, -50, 0, 0.1) : ba.db2linear : si.smoo;
freq = hslider("freq [unit:Hz]", 1000, 20, 2000, 1) : si.smoo;

process = vgroup("Oscillator", os.osc(freq) * vol);
