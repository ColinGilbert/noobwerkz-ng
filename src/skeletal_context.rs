use futures::executor::*;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::Cursor;
use std::rc::*;
use std::path::*;

pub struct SkeletalContext {
    pub skeleton: Rc<ozz_animation_rs::Skeleton>,
    pub animations: Vec<Rc<ozz_animation_rs::Animation>>,
    pub animations_idx_by_name: HashMap<String, usize>,
}

impl SkeletalContext {
    pub fn new(
        filepath: &Vec<&str>,
        skeleton_filename: &str,
        animation_filenames: &Vec<&str>,
    ) -> Self {
        let mut skeleton_filepath = PathBuf::new();
        for p in filepath {
            skeleton_filepath.push(p);
        }
        skeleton_filepath.push(skeleton_filename);
        let mut ar_skeleton = block_on(load_archive(skeleton_filepath.as_path())).unwrap();
        let mut ar_animations = Vec::new();
        let mut animations_idx_by_name = HashMap::<String, usize>::new();
        for a in animation_filenames {
            let mut anim_filepath = PathBuf::new();
            for p in filepath {
                anim_filepath.push(p);
            }
            anim_filepath.push(a);
            //println!("Getting animation {}", anim_filepath);
            ar_animations.push(block_on(load_archive(anim_filepath.as_path())).unwrap());
            let stripped_name = Path::new(a).file_stem().and_then(OsStr::to_str).unwrap().to_owned();
            animations_idx_by_name.insert(stripped_name, ar_animations.len() - 1);
        }
        
        let skeleton =
        Rc::new(ozz_animation_rs::Skeleton::from_archive(&mut ar_skeleton).unwrap());
        
        let mut animations = Vec::new();
        for mut a in ar_animations {
            animations.push(Rc::new(ozz_animation_rs::Animation::from_archive(&mut a).unwrap()));
        }

        Self {
            skeleton,
            animations,
            animations_idx_by_name
        }
    }

    pub fn get_anim_name_map(&self) -> HashMap<String, Rc<ozz_animation_rs::Animation>> {
        let mut results = HashMap::<String, Rc<ozz_animation_rs::Animation>>::new();

        for (n, i) in self.animations_idx_by_name.iter() {
            results.insert(n.to_string(), self.animations[*i].clone());
        }

        results
    }
}

async fn load_archive(
    path: &Path,
) -> Result<ozz_animation_rs::Archive<Cursor<Vec<u8>>>, ozz_animation_rs::OzzError> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?; // Not async! For compatible with reqwest.
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    return ozz_animation_rs::Archive::from_vec(buf);
}