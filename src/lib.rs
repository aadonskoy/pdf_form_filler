#[macro_use]
extern crate helix;

extern crate pdf_form_ids;
use pdf_form_ids::{Form, ValueError};

ruby! {
    class PdfFormFiller {
        def work() -> String {
        return "It work".to_string()
        }
    }
}

// Ruby hash is vec of tuples Vec::<(val, val)>
fn process_file(
    file_in: &str,
    file_out: &str,
    fileds: Option<Vec<(usize, &str)>>,
) -> Result<(), ValueError> {
    let mut form = Form::load(file_in).unwrap();
    Ok(())
}
