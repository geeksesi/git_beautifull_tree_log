use git2::Repository;
// fn my_cb( &oid:bool ) {
//     use git2::ObjectType::*;
//     let object = repo.find_object(oid, None).expect("no error");
//     match object.kind() {
//         Some(Tag) => print!("h"),
//         Some(Commit) => print!("h"),
//         Some(Tree) => print!("h"),
//         Some(Blob) => print!("h"),
//         Some(Any) => print!("h"),
//         None => print!("h"),
//     }
// }

fn run()  {
    let repo = match Repository::open("/home/geeksesi/public_html/Rust/git_beautifull_tree_log") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };
    // print!("{:#?}", repo);
    let odb = match repo.odb(){
        Ok(odb) => odb,
        Err(e) => panic!("failed to open: {}", e)
    };

    let elements = odb.foreach(|&oid|{
        // use git2::ObjectType::*;
        let object = repo.find_object(oid, None).expect("no error");
        print!("{:#?}", object);
        // match object.kind() {
        //     Some(Tag) => print!("{:#?}", Tag),
        //     Some(Commit) => print!("{:#?}", Commit),
        //     Some(Tree) => print!("{:#?}", Tree),
        //     Some(Blob) => print!("{:#?}", Blob),
        //     Some(Any) => print!("{:#?}", Any),
        //     None => print!("None"),
        // }
    });
    // Ok(2);

}

fn main()  {
    run();
}
