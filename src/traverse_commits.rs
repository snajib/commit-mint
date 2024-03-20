use git2::{Repository, Error};

pub fn traverse_commits(repo_path: &str) -> Result<(), Error> {
    let repo = Repository::open(repo_path)?;
    let head = repo.head()?;

    let branch_name = head.shorthand().unwrap_or("HEAD");
    let branch = repo.find_branch(branch_name, git2::BranchType::Local)?;
    let branch_ref = branch.get();

    let mut revwalk = repo.revwalk()?;
    revwalk.push(branch_ref.target().unwrap())?;

    for oid in revwalk {
        let branch_oid = oid?;
        let commit = repo.find_commit(branch_oid)?;
        let commit_message = commit.message().unwrap_or("branchOid");
        println!("Commit msg: {}", commit_message);

        let parent = commit.parent(0).ok();
        let tree_a = match parent {
            Some(parent_commit) => Some(parent_commit.tree()?),
            None => None
        };
        let tree_b = commit.tree()?;
        let diff = repo.diff_tree_to_tree(tree_a.as_ref(), Some(&tree_b), None)?;

        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            println!(" {}", String::from_utf8_lossy(line.content()));
            return true
        })?;
        println!("----------------")
    }
    Ok(())
}