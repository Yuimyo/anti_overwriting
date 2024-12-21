use crate::sys::fs as fs_imp;

pub fn rename() {
    fs_imp::rename();
}

// pub fn rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
//     fs_imp::rename(from.as_ref(), to.as_ref())
// }
