use pipewire as pw;

pub struct PipewireData {
    mainloop: pw::MainLoop,
    context: pw::Context<pw::MainLoop>,
    core: pw::Core,
    registry: pw::registry::Registry,
    listener: pw::registry::Listener,
    // stream: pw::stream::Stream,
    // properties: pw::Properties,
}

impl PipewireData {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mainloop = pw::MainLoop::new()?;
        let context = pw::Context::new(&mainloop)?;
        let core = context.connect(None)?;
        let registry = core.get_registry()?;

        let listener = registry
            .add_listener_local()
            .global(|global| println!("New global: {:?}", global))
            .register();

        mainloop.run();

        Ok(Self {
            mainloop,
            context,
            core,
            registry,
            listener,
        })
    }
}

