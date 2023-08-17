use super::SinkElement;
use gst::prelude::*;

impl SinkElement {
    pub fn init_autovideo(pipeline: &gst::Pipeline) -> Result<Self, glib::BoolError> {
        let sink = gst::ElementFactory::make("autovideosink")
            .name("autovideosink")
            .build()?;

        pipeline.add(&sink)?;
        Ok(Self { sink: sink })
    }
}
