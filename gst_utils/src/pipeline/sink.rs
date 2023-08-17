mod autovideo;
mod fake;
mod multi_file;

#[allow(unused)]
pub struct SinkElement {
    sink: gst::Element,
}

impl SinkElement {
    pub fn get_element(&self) -> &gst::Element {
        &self.sink
    }
}

impl Default for SinkElement {
    fn default() -> Self {
        let sink = gst::ElementFactory::make("autovideosink")
            .name("autovideosink")
            .build()
            .expect("Could not create sink element.");
        Self { sink: sink }
    }
}
