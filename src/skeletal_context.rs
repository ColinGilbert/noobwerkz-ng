use futures::executor::*;
use std::io::Cursor;
use std::sync::*;

pub struct SkeletalContext {
    pub skeleton: Arc<ozz_animation_rs::Skeleton>,
    pub animations: Vec<Arc<ozz_animation_rs::Animation>>,
}

impl SkeletalContext {
    pub fn new(
        filepath: String,
        skeleton_filename: String,
        animation_filenames: &Vec<String>,
    ) -> Self {
        let skeleton_filepath = filepath + &"/".to_owned() + &skeleton_filename;
        
        let mut ar_skeleton = block_on(load_archive(skeleton_filepath.as_str())).unwrap();
        let mut ar_animations = Vec::new();
        
        for a in animation_filenames {
            ar_animations.push(block_on(load_archive(a.as_str())).unwrap());
        }

        let skeleton =
            Arc::new(ozz_animation_rs::Skeleton::from_archive(&mut ar_skeleton).unwrap());
        
        let mut animations = Vec::new();
        for mut a in ar_animations {
            animations.push(Arc::new(ozz_animation_rs::Animation::from_archive(&mut a).unwrap()));
        }

        Self {
            skeleton,
            animations,
        }
    }
}

async fn load_archive(
    path: &str,
) -> Result<ozz_animation_rs::Archive<Cursor<Vec<u8>>>, ozz_animation_rs::OzzError> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(path)?; // Not async! For compatible with reqwest.
    let mut buf = vec![];
    file.read_to_end(&mut buf)?;
    return ozz_animation_rs::Archive::from_vec(buf);
}
