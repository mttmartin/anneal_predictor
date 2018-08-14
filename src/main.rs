struct AnnealSimulation {
	template: String,
	oligo: String,
	temperature: f64,
}

impl AnnealSimulation {
	fn deltaG_dinucleotide(&self, substr1: String, substr2: String) -> f64 {
		let mut deltaS = 0.0;
		let mut deltaH = 0.0;

		if substr1 ==  "AA" && substr2 == "TT" {
			deltaS =  -92.9;
			deltaH = -33.1;
		} else if substr1 == "AT" && substr2 == "TA" {
			deltaS =  -85.4;
			deltaH = -30.1;
		} else if substr1 == "TA" && substr2 == "AT" {
			deltaS =  -89.1;
			deltaH = -30.1;
		} else if substr1 == "CA" && substr2 == "GT" {
			deltaS =  -95.0;
			deltaH = -35.6;
		} else if substr1 == "GT" && substr2 == "CA" {
			deltaS =  -93.7;
			deltaH = -35.1;
		} else if substr1 == "CT" && substr2 == "GA" {
			deltaS =  -87.9;
			deltaH = -32.6;
		} else if substr1 == "GA" && substr2 == "CT" {
			deltaS =  -92.9;
			deltaH = -34.3;
		} else if substr1 == "CG" && substr2 == "GC" {
			deltaS =  -113.8;
			deltaH = -44.4;
		} else if substr1 == "GC" && substr2 == "CG" {
			deltaS =  -102.1;
			deltaH = -41.0;
		} else if substr1 == "GG" && substr2 == "CC" {
			deltaS =  -83.3;
			deltaH = -33.5;
		}


		/* Non Watson-Crick pairs
		From: Allawi, H. T., & SantaLucia, J. (1998). Thermodynamics of internal
		C.T mismatches in DNA. Nucleic Acids Research, 26(11), 2694–701.
		Retrieved from http://www.ncbi.nlm.nih.gov/pubmed/10090733 */

		else if substr1 == "AG" && substr2 == "TT" {
			deltaS = 3.8;
			deltaH = 4.2;
		} else if substr1 == "AT" && substr2 == "TG" {
			deltaS = -35.0;
			deltaH = -11.0;
		} else if substr1 == "CG" && substr2 == "GT" {
			deltaS = -49.0;
			deltaH = -17.0;
		} else if substr1 == "CT" && substr2 == "GG" {
			deltaS = -34.0;
			deltaH = -12.0;
		} else if substr1 == "GG" && substr2 == "CT" {
			deltaS = 43.5;
			deltaH = 14.0;
		} else if substr1 == "GG" && substr2 == "TT" {
			deltaS = 68.2;
			deltaH = 24.0;
		} else if substr1 == "GT" && substr2 == "CG" {
			deltaS = -51.5;
			deltaH = -18.0;
		} else if substr1 == "GA" && substr2 == "CT" {
			deltaS =  -92.9;
			deltaH = -34.3;
		} else if substr1 == "CG" && substr2 == "GC" {
			deltaS =  -113.8;
			deltaH = -44.4;
		} else if substr1 == "GC" && substr2 == "CG" {
			deltaS =  -102.1;
			deltaH = -41.0;
		} else if substr1 == "GG" && substr2 == "CC" {
			deltaS =  -83.3;
			deltaH = -33.5;
		}

		/* Non Watson-Crick pairs
		From: Allawi, H. T., & SantaLucia, J. (1998). Thermodynamics of internal
		C.T mismatches in DNA. Nucleic Acids Research, 26(11), 2694–701.
		Retrieved from http://www.ncbi.nlm.nih.gov/pubmed/10090733 */

		else if substr1 == "AG" && substr2 == "TT" {
			deltaS = 3.8;
			deltaH = 4.2;
		} else if substr1 == "TG" && substr2 == "AT" {
			deltaS = -7.1;
			deltaH = -0.42;
		} else if substr1 == "TG" && substr2 == "GT" {
			deltaS = -26.0;
			deltaH = -5.9;
		} else if substr1 == "TT" && substr2 == "AG" {
			deltaS = -22.0;
			deltaH = -5.4;
		}

		/* More non-Watson-Crick sets
		From: Allawi, H. T., & SantaLucia, J. (1998). Nearest neighbor
		thermodynamic parameters for internal G.A mismatches in DNA.
		Biochemistry, 37(8), 2170–9. doi:10.1021/bi9724873 */

		else if substr1 == "AA" && substr2 == "TG" {
			deltaS = -9.6;
			deltaH = -2.5;
		} else if substr1 == "AG" && substr2 == "TA" {
			deltaS = -9.6;
			deltaH = -2.9;
		} else if substr1 == "CA" && substr2 == "GG" {
			deltaS = -9.6;
			deltaH = -2.9;
		} else if substr1 == "CG" && substr2 == "GA" {
			deltaS = -55.2;
			deltaH = -16.7;
		} else if substr1 == "GA" && substr2 == "CG" {
			deltaS = -4.2;
			deltaH = -2.5;
		} else if substr1 == "GG" && substr2 == "CA" {
			deltaS = 13.4;
			deltaH = 2.1;
		} else if substr1 == "TA" && substr2 == "AG" {
			deltaS = 2.9;
			deltaH = 2.9;
		} else if substr1 == "TG" && substr2 == "AA" {
			deltaS = 31.0;
			deltaH = 13.0;
		}

		/* More mismatches
		From: Allawi, H. T., & SantaLucia, J. (1997). Thermodynamics of internal
		G.T mismatches in DNA. Nucleic Acids Research, 36(11), 10581–10594.
		Retrieved from http://www.ncbi.nlm.nih.gov/pubmed/10090733 */

		else if substr1 == "AG" && substr2 == "TT" {
			deltaS = 3.8;
			deltaH = 4.2;
		} else if substr1 == "AT" && substr2 == "TG" {
			deltaS = -35.0;
			deltaH = -10.0;
		} else if substr1 == "CG" && substr2 == "GT" {
			deltaS = -49.0;
			deltaH = -17.2;
		} else if substr1 == "CT" && substr2 == "GG" {
			deltaS = -33.0;
			deltaH = -12.0;
		} else if substr1 == "GG" && substr2 == "CT" {
			deltaS = 43.5;
			deltaH = 14.0;
		} else if substr1 == "GG" && substr2 == "TT" {
			deltaS = 68.2;
			deltaH = 24.0;
		} else if substr1 == "GT" && substr2 == "CG" {
			deltaS = -51.5;
			deltaH = -18.0;
		} else if substr1 == "GT" && substr2 == "TG" {
			deltaS = 40.0;
			deltaH = 17.0;
		} else if substr1 == "TG" && substr2 == "AT" {
			deltaS = -7.1;
			deltaH = -0.4;
		} else if substr1 == "TG" && substr2 == "GT" {
			deltaS = -26.0;
			deltaH = -5.9;
		} else if substr1 == "TT" && substr2 == "AG" {
			deltaS = -22.0;
			deltaH = -5.4;
		}

		/* More mismatches
		From: Allawi, H. T., & SantaLucia, J. (1998). Nearest-neighbor
		thermodynamics of internal A.C mismatches in DNA: sequence dependence
		and pH effects. Biochemistry, 37(26), 9435–44. doi:10.1021/bi9803729 */
		// These are for pH=7.0
		// TODO: Add in pH dependence? What about the others?

		else if substr1 == "AA" && substr2 == "TC" {
			deltaS = 19.0;
			deltaH = 9.6;
		} else if substr1 == "AC" && substr2 == "TA" {
			deltaS = 61.1;
			deltaH = 22.0;
		} else if substr1 == "CA" && substr2 == "GC" {
			deltaS = 15.0;
			deltaH = 7.9;
		} else if substr1 == "CC" && substr2 == "GA" {
			deltaS = -2.5;
			deltaH = 2.5;
		} else if substr1 == "GA" && substr2 == "CC" {
			deltaS = 59.4;
			deltaH = 22.0;
		} else if substr1 == "GC" && substr2 == "CA" {
			deltaS = -16.0;
			deltaH = -3.0;
		} else if substr1 == "TA" && substr2 == "AC" {
			deltaS = 33.0;
			deltaH = 14.0;
		} else if substr1 == "TC" && substr2 == "AA" {
			deltaS = 84.5;
			deltaH = 32.0;
		}


		/* More mismatches from: Peyret, N., Seneviratne, P. A., Allawi, H. T., & 
		Santalucia, J. (1999). Articles Nearest-Neighbor Thermodynamics and NMR 
		of DNA Sequences with Internal A.A, C.C, G.G, and T.T Mismatches. 
		Biochemistry, 38(12), 3468–3477. */
		// Assuming Table 2 is mislabeled and "∆H° (eu)" means "∆S° (eu)"

		else if substr1 == "AA" && substr2 == "TA" {
			deltaS = 7.1;
			deltaH = 5.0;
		} else if substr1 == "CA" && substr2 == "GA" {
			deltaS = -18.0;
			deltaH = -4.0;
		} else if substr1 == "GA" && substr2 == "CA" {
			deltaS = -41.0;
			deltaH = -12.0;
		} else if substr1 == "TA" && substr2 == "AA" {
			deltaS = 54.0;
			deltaH = 20.0;
		} else if substr1 == "AC" && substr2 == "TC" {
			deltaS = -18.0;
			deltaH = 0.0001; // Error checks for zero, was pm 2.1
		} else if substr1 == "CC" && substr2 == "GC" {
			deltaS = -30.0;
			deltaH = -6.3;
		} else if substr1 == "GC" && substr2 == "CC" {
			deltaS = 37.0;
			deltaH = 15.0;
		} else if substr1 == "TC" && substr2 == "AC" {
			deltaS = 68.6;
			deltaH = 26.0;
		} else if substr1 == "AG" && substr2 == "TG" {
			deltaS = -40.0;
			deltaH = -13.0;
		} else if substr1 == "CG" && substr2 == "GG" {
			deltaS = -64.0;
			deltaH = -21.0;
		} else if substr1 == "GG" && substr2 == "CG" {
			deltaS = -66.1;
			deltaH = -25.0;
		} else if substr1 == "TG" && substr2 == "AG" {
			deltaS = 15.0;
			deltaH = 6.7;
		} else if substr1 == "AT" && substr2 == "TT" {
			deltaS = -45.2;
			deltaH = -11.0;
		} else if substr1 == "CT" && substr2 == "GT" {
			deltaS = -66.1;
			deltaH = -21.0;
		} else if substr1 == "GT" && substr2 == "CT" {
			deltaS = -35.0;
			deltaH = -9.2;
		} else if substr1 == "TT" && substr2 == "AT" {
			deltaS = -6.3;
			deltaH = 0.8;
		} else {
			println!("Warning: Duplex pair not found: {:?} and {:?}", substr1, substr2);
		}

		// Units above are in J/L*mol for ease of entry--convert to kJ/mol
		deltaS = deltaS/1000.0;
		let deltaG = deltaH - self.temperature * deltaS;
		deltaG
	}

fn initiation_termination_deltaG(&self, template_dinucleotide: String, oligo_dinucleotide: String) -> f64 {
	if (template_dinucleotide[0..0].to_string() == "A" && oligo_dinucleotide[0..0].to_string() == "T") || (template_dinucleotide[0..0].to_string() == "T" && oligo_dinucleotide[0..0].to_string() == "A") {
		let deltaH = 9.62;
		let deltaS = 17.15/1000.0;
		let deltaGinit = deltaH - (self.temperature*deltaS);
		deltaGinit
	} else if (template_dinucleotide[0..0].to_string() == "G" && oligo_dinucleotide[0..0].to_string() == "C") || (template_dinucleotide[0..0].to_string() == "C" && oligo_dinucleotide[0..0].to_string() == "G") {
		let deltaH = 0.4184;
		let deltaS = -11.72/1000.0;
		let deltaGinit = deltaH - (self.temperature*deltaS);
		deltaGinit
	} else {
		0.0
	}
}

fn calc_deltaG(&self, template_piece: String) -> f64{
	let mut deltaG = 0.0;
	println!("template piece: {:?}", template_piece);
	for i in 0..(self.oligo.len()-1) {
		println!("here");
		let mut initiation_deltaG = 0.0;
		let template_dinucleotide = self.template[i..i+2].to_string();
		let oligo_dinucleotide = self.oligo[i..i+2].to_string();
		let mut dinucleotide_deltaG = self.deltaG_dinucleotide(template_dinucleotide.clone(), oligo_dinucleotide.clone());

		if i == 0 {
			initiation_deltaG = self.initiation_termination_deltaG(template_dinucleotide[0..0].to_string(), oligo_dinucleotide[0..0].to_string());
		} else if i == (self.oligo.len()-2) {
			initiation_deltaG = self.initiation_termination_deltaG(template_dinucleotide[1..1].to_string(), oligo_dinucleotide[1..1].to_string());
		}
		deltaG = deltaG + dinucleotide_deltaG + initiation_deltaG;
		println!("Current deltaG: {:?}", deltaG);

	}
	deltaG
}

	fn lowest_deltaG(&self) -> f64 {
		let mut lowestDeltaG = 0.0;
		for i in 0..(self.template.len() - self.oligo.len()) {
			let template_piece = self.template[i..(self.oligo.len()+i)].to_string();
			let currentDeltaG = self.calc_deltaG(template_piece);
			if currentDeltaG < lowestDeltaG {
				lowestDeltaG = currentDeltaG;
			}
		}
		lowestDeltaG
	}
}


fn main() {
	//let seq1 = "CCGATATTTCACGAATTAATTTTCACGTTCACGTTCACGTTCAGTTCA".to_string();
	//let seq2 = "AATTTTAAATGCAAGT".to_string();
	let seq1 = "CGTTGAA".to_string();
	let seq2 = "GCAACT".to_string();
	let temperature = 310.15;
	let simulation = AnnealSimulation { template: seq1, oligo: seq2, temperature };
	let lowest_delta_G = simulation.lowest_deltaG();

	println!("Lowest deltaG is {:?}", lowest_delta_G);
}
