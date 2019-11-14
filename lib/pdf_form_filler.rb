require "helix_runtime"

begin
  require "pdf_form_filler/native"
rescue LoadError
  warn "Can't load pdf_form_filler. Please run `rake build`."
end