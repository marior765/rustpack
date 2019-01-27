
use std::error::Error;

pub impl Error for RustpackError {

    fn new(&self, message: String) -> RustpackError {
        self.details = unimplemented!();
        self.missing = unimplemented!();
		self.origin = unimplemented!();
		self.dependencies = unimplemented!();
		self.module = unimplemented!();

        // Error.captureStackTrace(this, this.constructor);
    }

    fn inspect(&self) -> String {
        // return this.stack + (this.details ? `\n${this.details}` : "");
    }
}
