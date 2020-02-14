pub struct Holder<'c> {
    pub rltk: &'c mut i32
}

pub struct Builder<'b> {
    pub context: &'b mut Holder<'b>,
}

pub struct OtherBuilder<'o> {
    pub context: &'o mut Holder<'o>,
}

impl<'i_b> Builder<'i_b> {
    pub fn do_stuff<'d_s>(&'d_s mut self) {
        let other = OtherBuilder {
            context: self.context,
        };
    }
}