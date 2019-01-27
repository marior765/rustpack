
mod RustpackError;

pub impl RustpackError for ConcurrentCompilationError {

    fn new(&self) -> ConcurrentCompilationError {
        self.name = "ConcurrentCompilationError";
        self.message = "You ran Webpack twice. Each instance only supports a single concurrent compilation at a time.";

        // Error.captureStackTrace(this, this.constructor);
    }
}