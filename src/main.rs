use time;
use git2::{Commit, ObjectType, Repository};
use git2::BranchType::Local;

fn main() {
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());
    let repo = Repository::open(repo_root.as_str()).expect("Couldn't open repository");
    let bb = get_branches(&repo);
    println!("{:?}", bb.unwrap());
    // checkout_branch(&repo, "master");
}

fn get_branches(repo: &Repository) -> Result<Vec<String>, git2::Error>{
    let mut result: Vec<String> = vec![];
    let branches = repo.branches(Some(Local))?;
    for b in branches {
        match b {
            Ok((b, b_t)) => result.push(String::from(b.name().unwrap().unwrap())),
            Err(e) =>println!("error")
        }
    }
    return Ok(result);
}

fn checkout_branch(repo: &Repository, branch: &str) {
    let (obj, reference) = repo.revparse_ext(branch).unwrap();
    repo.checkout_tree(&obj, None).expect("Can not chechout working directory");

    let reff = reference.unwrap();
    let ref_name = reff.name().unwrap();

    println!("{}", ref_name);

    repo.set_head(ref_name);

}