impl A {}
impl A for B {}

impl<T> A {}
impl A where T: B {}
impl A { #![a] }
impl A { fn a(); }

impl<T> A where T: B { #![a] fn a(); fn b(); }

unsafe impl A for B {}

impl<T> A for B {}
impl A for B where T: C {}
impl A for B { #![a] }
impl A for B { fn a(); }

impl<T> A for B where T: C { #![a] fn a(); fn b(); }
