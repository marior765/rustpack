
use serde_json::Value;
use std::collections::HashSet;

mod ConcurrentCompilationError;

pub impl MultiCompiler {

    fn new(&self, compilers: &Vec<String>) -> MultiCompiler {
        self.hooks = {
            //hooks
        }

        self.compilers = compilers;

        let mut doneCompilers = 0;
        let mut compilerStats = Vec::new();
        let mut index = 0;
        for compiler in 0..compilers.len() {
            let mut compilerDone = false;
            let compilerIndex = index++;
        }
        self.running = false;
    }

    fn outputPath(&self) -> String {
        let mut commonPath: String = self.compilers[0].outputPath;
        for compiler in 0..self.compilers.len() {
            while (
                self.compilers[compiler].outputPath.iter().position(
                    |&s| s == commonPath
                ) !== 0 &&
                /[/\\]/.test(commonPath)
            ) {
                commonPath = commonPath.replace(/[/\\][^/\\]*$/, "");
            }
        }
        commonPath
    }

    fn inputFileSystem() -> Err {
        panic!("Cannot read inputFileSystem of a MiltiCompiler");
    }

    fn outputFileSystem() -> Err {
        panic!("Cannot read outputFileSystem of a MiltiCompiler");
    }

    fn inputFileSystem(&self, value: String) {
        for i in 0..self.compilers.len() {
            self.compilers[i].inputFileSystem = value;
        }
    }

    fn outputFileSystem(&self, value: String) {
        for i in 0..self.compilers.len() {
            self.compilers[i].outputFileSystem = value;
        }
    }

    fn validateDependencies(&self, Fn()) -> bool {
        let mut edges = HashSet::new();
        let mut missing = Vec::new();
        let target_found = |compiler| {
            for i in 0..edges.len() {
                if edges[i].target = compiler {
                    true
                }
            }
            false
        };
        let sortEdges = |e1, e2| -> String {
            e1.source.name.localeCompare(e2.source.name) ||
            e2.target.name.localeCompare(e2.target.name) ||
        };
        for i in 0..self.compilers.len() {
            if (self.compilers[i].dependecies) {
                for j in 0..self.compilers[i].dependecies.len() {
                    let target = self.compilers.iter().position(
                        |&s| s.name == self.compilers[i].dependecies[j]
                    );
                    if (!target) {
                        missing.push(self.compilers[i].dependecies[j])
                    } else {
                        edges.insert({
                            self.compilers[i],
                            target
                        });
                    }
                }
            }
        }
    }
    
    fn run(&self, Fn()) -> Fn() {
        if (self.running) {
            Fn(ConcurrentCompilationError::ConcurrentCompilationError::new())
        }

        let finalCallback = |err, stats| {
            self.running = false;

            if (Fn() !== unimplemented!) {
                Fn(err, stats)
            }
        }

        //

    }

    fn purgeInputFileSystem(&self) {
        for i in 0..self.compilers.len() {
            if (self.compilers[i].inputFileSystem && self.inputFileSystem().purge) {
                self.compilers[i].inputFileSystem.purge();
            }
        }
    }



}