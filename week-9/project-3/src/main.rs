use std::fs::File;
use std::io::Write;

fn main() {
let commissioners = [
"Aigbogun Alamba Daudu",
"Murtala Afeez Bendu",
"Okorocha Calistus Ogbona",
"Adewale Jimoh Akanbi",
"Osazuwa Faith Etiye",
];

let ministries = [
"Internal Affairs",
"Justice",
"Defense",
"Power & Steel",
"Petroleum",
];

let zones = [
"South West",
"North East",
"South South",
"South West",
"South East",
];

let mut file = File::create("efcc_merge.txt")
.expect("Could not create file");

writeln!(
file,
"Commissioner | Ministry | Zone"
)
.expect("Could not write to file");

println!("EFCC MERGED RECORDS");
println!("Commissioner | Ministry | Zone");

for i in 0..commissioners.len() {
println!(
"{} | {} | {}",
commissioners[i], ministries[i], zones[i]
);

writeln!(
file,
"{} | {} | {}",
commissioners[i], ministries[i], zones[i]
)
.expect("Could not write to file");
}
println!("Records saved to efcc_merge.txt");
}
