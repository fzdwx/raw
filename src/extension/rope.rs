use ropey::RopeSlice;

pub trait RopeSliceEx<'a> {
    /// repo slice to string
    fn get_string(&self) -> String;
}

impl<'a> RopeSliceEx<'a> for RopeSlice<'a> {
    fn get_string(&self) -> String {
        format!("{}", self)
    }
}
