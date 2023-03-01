use std::env;
use bio::io::fasta;
use std::io::Write;
use bio::data_structures::suffix_array::suffix_array;
use bio::alignment::sparse::hash_kmers;
use std::collections::BTreeMap;
//use std::collections::HashMap;
use bincode;
use std::fs::File;
use bincode::serialize;
use std::time::Instant;
use serde::{Serialize, Deserialize};



fn main() -> std::io::Result<()> {
    let before = Instant::now();
    let args: Vec<String> = env::args().collect();
    let reference: &str = &args[1];
    let k = &args[2];
    let output = &args[3];
    let preftab:u32 = k.parse().unwrap();
    //println!("{:?}",prefixtable(reference,preftab));
    
    //buildsa(reference,preftab, output).unwrap();
    let results = buildsa(reference, preftab);
    println!("Elapsed time: {:.2?}", before.elapsed());
    let serialized1 = serialize(&results).unwrap();
    //println!("{:?}",results);
    let mut file = File::create(output)?;
    file.write_all(&serialized1)?;
    
    //results
    Ok(())
    
    
}

#[derive(Serialize, Deserialize, Debug)]
struct MyResults {
    sequence:Vec<u8>,
    suffix_array: Vec<usize>,
    //k: u32,
    prefix_table: BTreeMap<Vec<u8>, Vec<u32>>,
}



fn buildsa(reference:&str,preftab:u32) -> MyResults {
    let suffix_array = sa(reference);
    let prefixtable = prefixtable(reference,preftab);
    let seqs = getsequence(reference);

    let results = MyResults {
        sequence:seqs.clone(),
        suffix_array: suffix_array.clone(),
        //k: preftab,
        prefix_table: prefixtable.clone()};
    results

}

fn sa(reference: &str) -> Vec<usize> {

    let reader = fasta::Reader::from_file(reference).unwrap();
    //let mut sa_vec = Vec::new();
    //let mut sa_map = HashMap::new();
    let mut sa_result:Vec<Vec<usize>> = Vec::new();
    //let text5 = Vec::new();

    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");

        let text = record.seq().to_vec();
        let text2 = String::from_utf8(text).unwrap();
        let text3 = String::from("$");
        //let text3: String = text2.copy().push('$');
        let text4 = text2+&text3;
        //println!("The input string: \n {}",text3);
        //let text4 = text3.as_bytes();
        let text5 = text4.as_bytes().to_owned();
        //println!("The input string (bytes): \n {:?}",text3);
        //println!("The suffix array: \n {:?}",sa);
        //sa_vec.push(sa.to_owned());
        let sa = suffix_array(&text5);
        sa_result.push(sa);

    }
    
    sa_result.concat()
    //write(output, &sa);
    //write(output, &map);
    
}


fn prefixtable(reference: &str,preftab:u32) -> BTreeMap<Vec<u8>, Vec<u32>> {
    //let file = File::open(reference)?;
    //let reader = BufReader::new(file);
    //let mut fasta_reader = fasta::Reader::new(reader);
    let reader = fasta::Reader::from_file(reference).unwrap();
    let mut sequences = Vec::new();

    // Iterate over all records in the fasta file
    for record in reader.records() {
        let record = record.expect("Error reading record");
        let sequence = record.seq();
        sequences.push(sequence.to_owned());
    }
    //let mut kmer_vec = Vec::new();
    let mut map = BTreeMap::new();
    //let mut map_vec = Vec::new();
    for seq in sequences {
        let seq1 = seq.to_owned();
        let kmer = hash_kmers(&seq1,preftab.try_into().unwrap());
        //kmer_vec.push(kmer.to_owned());
        for (km, pos) in kmer{
            map.insert(km.to_owned(), pos.to_owned());
            
        }
        
        
    }
    map
    //map_vec.push(map.to_owned());
    //map_vec
    
}

fn getsequence(reference:&str) -> Vec<u8> {
    let reader = fasta::Reader::from_file(reference).unwrap();
    //let mut sequences = HashMap::new();
    let mut sequence = Vec::new();
    for record in reader.records() {
        let record = record.expect("Error reading record");
        let text = record.seq().to_vec();
        let text2 = String::from_utf8(text).unwrap();
        let text3 = String::from("$");
        //let text3: String = text2.copy().push('$');
        let text4 = text2+&text3;
        //println!("The input string: \n {}",text3);
        //let text4 = text3.as_bytes();
        let text5 = text4.as_bytes().to_owned();

        //let sequence = record.seq();
        //let id = record.id();
        //sequences.insert(id.to_owned(),text5.to_owned());
        sequence.push(text5.to_owned());
    }
    sequence.concat()
}

