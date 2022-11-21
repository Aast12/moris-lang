use std::path::PathBuf;

use codegen::generate;
use codegen::manager::Manager;

use super::virtual_machine::VirtualMachine;

pub struct Runner {
    pub path: String,
    pub out_path: String,
    pub manager: Manager,
}

impl Runner {
    pub fn new_managed(path: &str, manager: Manager) -> Result<Runner, String> {
        let path_buf = PathBuf::from(path);

        if path_buf.is_file() {
            let path = path_buf.as_os_str().to_str().unwrap();

            Ok(Runner {
                path: path.to_string(),
                out_path: String::from("out.o"),
                manager,
            })
        } else {
            Err(format!("Path {path} is not a file!"))
        }
    }

    pub fn new(path: &str) -> Result<Runner, String> {
        let path_buf = PathBuf::from(path);

        if path_buf.is_file() {
            let path = path_buf.as_os_str().to_str().unwrap();

            Ok(Runner {
                path: path.to_string(),
                out_path: String::from("out.o"),
                manager: Manager::new(),
            })
        } else {
            Err(format!("Path {path} is not a file!"))
        }
    }

    pub fn with_output_path(&mut self, obj_path: &str) -> &mut Self {
        self.out_path = String::from(obj_path);
        self
    }

    pub fn compile(&mut self) {
        generate(self.path.as_str(), &mut self.manager);
        self.manager.dump(&PathBuf::from(self.out_path.as_str()));
    }

    pub fn clean(&mut self) {
        self.manager.reset();
    }

    pub fn run(&mut self) -> VirtualMachine {
        let mut vm = VirtualMachine::load(&self.out_path.as_str());
        vm.execute();

        vm
    }
}
