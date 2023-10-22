pub trait Tickable {
    fn tick(&mut self);

    fn tick_times(&mut self, n: usize) {
        for _ in 0..n {
            self.tick();
        }
    }
}
