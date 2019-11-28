#![recursion_limit = "1024"]

#[macro_use]
extern crate helix;

extern crate pdf_form_ids;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

use pdf_form_ids::Form;

ruby! {
    class PdfFormFiller {
        def work() -> String {
            return "It work".to_string()
        }

        def process_pdf(file_in: String, file_out: String, dataset: HashMap<String, String>) {
            println!("passing to inner fnction: {:?}", &file_in);
            process_file(&file_in, &file_out, dataset);
        }
    }
}

fn process_file(file_in: &str, file_out: &str, dataset: HashMap<String, String>) {
    let path = Path::new(&file_in);
    let mut form = Form::load(&path).unwrap();

    println!("read");

    let mut fields = HashMap::new();
    for ind in 0..form.len() {
        if let Some(name) = form.get_name(ind) {
            &fields.insert(name, ind);
        }
    }

    for (title, val) in dataset.iter() {
        if let Some(field_id) = fields.get(title) {
            // Only text field for now
            form.set_text(*field_id as usize, String::from(val))
                .expect("Error with setting value");
        }
    }
    println!("{:?}", fields);
}
