use crate::Register;

type BoxedReader = Box<dyn Fn(u8) -> u8>;
type BoxedTransformer = Box<dyn Fn(u8, u8) -> Option<u8>>;
type BoxedObserver = Box<dyn Fn(u8)>;

pub struct IORegister {
    value: u8,
    reader: BoxedReader,
    transformer: BoxedTransformer,
    hooks: Vec<BoxedObserver>,
}

impl IORegister {
    pub fn new() -> Self {
        Self {
            value: 0,
            reader: Box::new(|value| value),
            transformer: Box::new(|_, value| Some(value)),
            hooks: Vec::new(),
        }
    }

    pub fn add_transformer<F>(&mut self, transformer: F)
    where
        F: Fn(u8, u8) -> Option<u8> + 'static,
    {
        self.transformer = Box::new(transformer);
    }

    pub fn add_hook<F>(&mut self, hook: F)
    where
        F: Fn(u8) + 'static,
    {
        self.hooks.push(Box::new(hook));
    }

    pub fn add_reader<F>(&mut self, reader: F)
    where
        F: Fn(u8) -> u8 + 'static,
    {
        self.reader = Box::new(reader);
    }

    pub fn read(&self) -> u8 {
        (self.reader)(self.value)
    }

    pub fn write(&mut self, value: u8) {
        let transformed_opt = (self.transformer)(self.value, value);
        let Some(transformed) = transformed_opt else {
            return;
        };

        self.value = transformed;

        for hook in self.hooks.iter() {
            hook(self.value);
        }
    }

    pub fn write_unchecked(&mut self, value: u8) {
        self.value = value;
        for hook in self.hooks.iter() {
            hook(self.value);
        }
    }

    pub fn write_unhooked(&mut self, value: u8) {
        self.value = value;
    }
}

impl Register for IORegister {
    fn read(&self) -> u8 {
        self.read()
    }

    fn write(&mut self, value: u8) {
        self.write(value);
    }
}

impl Default for IORegister {
    fn default() -> Self {
        Self::new()
    }
}
