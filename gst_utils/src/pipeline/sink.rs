use gst::prelude::*;

#[allow(unused)]
pub struct SinkElement {
    sink: gst::Element,
}

impl SinkElement {
    pub fn new(pipeline: &gst::Pipeline) -> Result<Self, glib::BoolError> {
        // match format.as_str() {
        //     "auto" => _,
        //     _ => None,
        // }
        let sink = gst::ElementFactory::make("autovideosink")
            .name("autovideosink")
            .build()
            .expect("Could not create sink element.");
        pipeline.add(&sink).unwrap();
        Ok(Self { sink: sink })
    }

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
