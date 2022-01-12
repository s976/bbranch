use time;
use git2::{Commit, ObjectType, Repository};
use git2::BranchType::Local;

fn main() {
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());
    let repo = Repository::open(repo_root.as_str()).expect("Couldn't open repository");
    println!("{} state={:?}", repo.path().display(), repo.state());
    let c = find_last_commit(&repo).expect("Couldn't find last commit");
    display_commit(&c);
    list_branches(&repo);
    checkout_branch(&repo, "master");
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

fn display_commit(commit: &Commit) {
    let timestamp = commit.time().seconds();
    let tm = time::at(time::Timespec::new(timestamp, 0));
    println!("commit {}\nAuthor: {}\nDate:   {}\n\n    {}",
             commit.id(),
             commit.author(),
             tm.rfc822(),
             commit.message().unwrap_or("no commit message"));
}

fn list_branches(repo: &Repository) -> Result<(), git2::Error>{
    let branches = repo.branches(Some(Local))?;
    for b in branches {
        match b {
            Ok((b, b_t)) => println!("{:?}", b.name().unwrap().unwrap()),
            Err(e) =>println!("error")
        }
    }
    Ok(())
}

fn checkout_branch(repo: &Repository, branch: &str) {
    let (obj, reference) = repo.revparse_ext(branch).unwrap();
    repo.checkout_tree(&obj, None).unwrap();
    match reference {
        Some(r) => println!("{:?}", r.name().unwrap()),
        _ => println!("what is that?")
    }

    repo.set_head(&*("refs/heads/".to_owned() + branch));

}