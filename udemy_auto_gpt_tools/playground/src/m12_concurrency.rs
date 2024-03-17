#[cfg(test)]
#[allow(dead_code)]
mod test {

    use std::fs::{File, OpenOptions};
    use std::io::prelude::*;
    use std::sync::{Arc, Mutex};
    use std::thread::{JoinHandle, spawn};

    // cargo test tests_concurrency -- --nocapture
    #[test]
    fn tests_concurrency() {
        println!("\n---> mod 12 tests_concurrency \n");

        let file_mutex = Arc::new(Mutex::new(OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open("inc.txt")
            .unwrap()
        ));

        let mut handles = vec![];

        for i in 0..10  {
            let file_mutex = Arc::clone(&file_mutex);
            let handle = spawn(move || {
                let mut file = file_mutex.lock().unwrap();
                writeln!(file, "{}", i).unwrap();
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap()
        }

    }
}
