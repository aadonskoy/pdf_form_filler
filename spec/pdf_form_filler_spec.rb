require "pdf_form_filler"

RSpec.describe PdfFormFiller do
  it "does something useful" do
    expect(PdfFormFiller.work).to eq("It work")
  end

  it "process_pdf" do
    PdfFormFiller.process_pdf(
      "spec/support/test_simple_pdf.pdf",
      "out.pdf",
      {"Text2" => "www", "2 Question line text" => "Something you need to check"}
    )
    expect(true). to be
  end
end
