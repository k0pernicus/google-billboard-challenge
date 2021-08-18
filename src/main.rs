// Based on the book "Androids", from Chet Haase.
// For the complete puzzle, you can read this: https://www.seroundtable.com/archives/000644.html,
// or just google "Google Billboard Puzzle"

use std::str::FromStr;

use primes::is_prime;

// Thanks to NASA: https://apod.nasa.gov/htmltest/gifcity/e.2mil
const RAW_E: &'static str = r"
2.718281828459045235360287471352662497757247093699959574966
967627724076630353547594571382178525166427427466391932003059
921817413596629043572900334295260595630738132328627943490763
233829880753195251019011573834187930702154089149934884167509
244761460668082264800168477411853742345442437107539077744992
069551702761838606261331384583000752044933826560297606737113
200709328709127443747047230696977209310141692836819025515108
657463772111252389784425056953696770785449969967946864454905
987931636889230098793127736178215424999229576351482208269895
193668033182528869398496465105820939239829488793320362509443
117301238197068416140397019837679320683282376464804295311802
328782509819455815301756717361332069811250996181881593041690
351598888519345807273866738589422879228499892086805825749279
610484198444363463244968487560233624827041978623209002160990
235304369941849146314093431738143640546253152096183690888707
016768396424378140592714563549061303107208510383750510115747
704171898610687396965521267154688957035035402123407849819334
321068170121005627880235193033224745015853904730419957777093
503660416997329725088687696640355570716226844716256079882651
787134195124665201030592123667719432527867539855894489697096
409754591856956380236370162112047742722836489613422516445078
182442352948636372141740238893441247963574370263755294448337
998016125492278509257782562092622648326277933386566481627725
164019105900491644998289315056604725802778631864155195653244
258698294695930801915298721172556347546396447910145904090586
298496791287406870504895858671747985466775757320568128845920
541334053922000113786300945560688166740016984205580403363795
376452030402432256613527836951177883863874439662532249850654
995886234281899707733276171783928034946501434558897071942586
398772754710962953741521115136835062752602326484728703920764
310059584116612054529703023647254929666938115137322753645098
889031360205724817658511806303644281231496550704751025446501
172721155519486685080036853228183152196003735625279449515828
";

// The length of a number - declared in Google puzzle description
const LENGTH_OF_NUMBER: usize = 10;

struct E {
    index: usize,      // Declares where we are (like a custom iterator ~)
    numbers: Vec<u32>, // Contains all the numbers from Euler's number, in a single vector
}

impl std::str::FromStr for E {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let index = 0;
        // Skip the first 2 characters
        let numbers = s
            .chars()
            .skip(3) // We don't care about the '2.' of the Euler's number
            .map(|c| c.to_digit(10)) // Everything is a number
            .filter(|opt| opt.is_some()) // Remove possible errors due to new lines
            .map(|opt| opt.unwrap()) // Now remove the container
            .collect::<Vec<u32>>();
        Ok(E { index, numbers })
    }
}

impl E {
    // Returns the number (length of 10th) at stored index
    fn get_next_number(&mut self, length_of_number: usize) -> u64 {
        let mut number = 0u64;
        let mut iter = self.numbers[self.index..self.index + length_of_number].iter();
        while let Some(value) = iter.next() {
            number = (number * 10) + (*value as u64);
        }
        self.index += 1;
        number
    }
    // Returns the number (length of 10th) at stored index, and its internal sum
    fn sum_of_next_number(&mut self, length_of_number: usize) -> (u64, u32) {
        let mut number = 0u64;
        let mut iter = self.numbers[self.index..self.index + length_of_number].iter();
        while let Some(value) = iter.next() {
            number = (number * 10) + (*value as u64);
        }
        let sum = self.numbers[self.index..self.index + 10].iter().sum();
        self.index += 1;
        (number, sum)
    }
}

fn main() {
    // First part, find the prime number
    {
        let mut e: E = E::from_str(RAW_E).unwrap();
        let mut found_prime: Option<u64> = None;
        loop {
            let number = e.get_next_number(LENGTH_OF_NUMBER);
            if number == 0 {
                break;
            }
            if is_prime(number) {
                found_prime = Some(number);
                break;
            }
        }
        match found_prime {
            None => {
                println!("Oops, something wrong here... did not found the prime number!");
                std::process::exit(1);
            }
            Some(prime) => println!("> website is www.{}.com", prime),
        }
    }
    // Second part, find the password for linux.com
    {
        let mut e: E = E::from_str(RAW_E).unwrap();
        // Already known numbers
        let not_included: [u64; 4] = [7182818284, 8182845904, 8747135266, 7427466391];
        let sum_to_found: u32 = 49; // I found out (after a long time)
                                    // that if you sum the digits inside the number below, all equal 49 (!)
        loop {
            let (number, sum) = e.sum_of_next_number(LENGTH_OF_NUMBER);
            if sum == sum_to_found && !not_included.contains(&number) {
                println!("> in linux.com, '{}' could be the password", number);
                break;
            }
        }
    }
}
