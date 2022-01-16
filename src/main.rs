use git2::{Repository};
use git2::BranchType::Local;
use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use console::Term;
use std::panic;
use std::process;


fn main() {
    // set_panic_hook();
    let repo_root = std::env::args().nth(1).unwrap_or(".".to_string());
    let repo = Repository::discover(repo_root.as_str()).expect("Couldn't open repository");
    let bb = get_branches(&repo);
    let bb = bb.unwrap();
    let selected = select(&bb);
    match selected {
        Ok(opt) => {
            match opt {
                Some(ind) => checkout_branch(&repo, &bb[ind][..]),
                None => println!("Bye")
            }
        },
        Err(_) => println!("Error!")
    }
}

fn set_panic_hook() {
    panic::set_hook(Box::new(|_| {
        println!("Error. Exiting.");
        process::exit(1);
    }));
}

fn get_branches(repo: &Repository) -> Result<Vec<String>, git2::Error>{
    let mut result: Vec<String> = vec![];
    let branches = repo.branches(Some(Local))?;
    for b in branches {
        match b {
            Ok((b, _)) => {
                result.push(String::from(b.name().unwrap().unwrap()))
            },
            Err(_) =>println!("error!")
        }
    }
    return Ok(result);
}

fn checkout_branch(repo: &Repository, branch: &str) {
    let (obj, reference) = repo.revparse_ext(branch).unwrap();
    repo.checkout_tree(&obj, None).expect("Can not chechout working directory");

    let reff = reference.unwrap();
    let ref_name = reff.name().unwrap();

    repo.set_head(ref_name).unwrap();
}

fn select(items: &Vec<String>) -> std::io::Result<Option<usize>> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    return Ok(selection);
}