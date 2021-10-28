fn main() {
  let data = "\
    title,author,year
    Animal Farm,George Orwell,1945
    The art of war,Sun Tzu,1910
    Invailid,Field
    Not,is a,number
    ";

  process_csv(data);
}

fn process_csv(data: &str) {
  let records = data.lines();

  for (idx, record) in records.enumerate() {
    // Skips headline and lines with only whitespace.
    if idx == 0 || record.trim().len() == 0 {
      continue;
    }

    let fields: Vec<_> = record
      // split record into fields
      .split(',')
      // remove whitespace for each field
      .map(|field| field.trim())
      // build collection of fields [...]
      .collect();

    // Ignore incomplete fields
    if fields.len() < 3 {
      continue;
    }

    let (name, author) = (fields[0], fields[1]);
    // Attemps to parse field as a floating point number.
    if let Ok(year) = fields[2].parse::<f32>() {
      println!("{} - {} #{}", name, author, year);
    }
  }
}
