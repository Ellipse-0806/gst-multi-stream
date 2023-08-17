use super::SinkElement;
use gst::prelude::*;

impl SinkElement {
    pub fn init_fake(pipeline: &gst::Pipeline) -> Result<Self, glib::BoolError> {
        let sink = gst::ElementFactory::make("fakesink")
            .name("fakesink")
            .build()?;

        pipeline.add(&sink)?;
        Ok(Self { sink: sink })
    }
}
