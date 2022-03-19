use rush_theming::Theme;
struct RushStruct {
    value: &str,
}
trait RushTrait {
    fn foo(&self, foo: Foo, bar: &mut Bar) {
        Theme::default().show(|tokens| {
            let app = tokens.app;
            let items = vec![1, 2, 3, 4];
            run_app(app, items);
        })
    }
    pub fn bar(&mut self) {}
}
impl RushTrait for RushStruct {
    pub fn bar(&mut self) {
        self.value = "Hello World";
    }
}
