fn a();
fn a() {}

const fn a();
async fn a();
unsafe fn a();
extern fn a();
extern "C" fn a();

fn a<>();
fn a<A>();
fn a<A, B>();
fn a<A, B,>();

fn a(self);
fn a(#[a] #[b] self);
fn a(self,);

fn a(&self);
fn a(&'a self);
fn a(&mut self);
fn a(&'a mut self);

fn a(self: u32);
fn a(mut self: u32);

fn a(&self, a: u32);
fn a(&self, a: u32, b: u32,);
fn a(#[a] #[b] a: u32);
fn a(...);
fn a(u32);
fn a(a: u32, ..., u32);

fn a() where {}

fn a() where T: A {}
fn a() where T: {}
fn a() where for<'a, 'b> T: A {}
fn a() where 'a: 'b {}

fn a() where T: A, {}

fn a() where T: A + B +, 'a: 'b + 'c + {}

const async unsafe extern "C" fn a<A, B,>(&'a mut self, a: u32, b: u32,) -> u32 {}
