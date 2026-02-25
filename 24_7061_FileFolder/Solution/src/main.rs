fn main(){
    let curdir = std::env::current_dir().unwrap();
    println!("Current directory: {}", curdir.display());
    ////////////////////////////////////////////////////
    let newdir = curdir.join("newfolder");
    if newdir.exists() {
        println!("Directory already exists: {}", newdir.display());
    } else {
        std::fs::create_dir(&newdir).unwrap();
        println!("Created directory: {}", newdir.display());
    }
    ////////////////////////////////////////////////////
    let newfile = newdir.join("newfile.txt");
    if newfile.exists() {
        println!("File already exists: {}", newfile.display());
    } else {
        std::fs::write(&newfile, "Hello, world!").unwrap();
        println!("Created file: {}", newfile.display());
    }
    ////////////////////////////////////////////////////
    let contents = std::fs::read_to_string(&newfile).unwrap();
    println!("File contents: {}", contents);
}