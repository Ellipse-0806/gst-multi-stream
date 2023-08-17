use super::SinkElement;
use gst::prelude::*;

impl SinkElement {
    pub fn init_multifile(
        location: &str,
        pipeline: &gst::Pipeline,
    ) -> Result<Self, glib::BoolError> {
        let encoder = gst::ElementFactory::make("jpegenc")
            .name("jpegencoder")
            .build()
            .expect("Could not create encoder element.");

        let sink = gst::ElementFactory::make("multifilesink")
            .name("multifilesink")
            .property("location", format!("{}/{}", location, "%05d.jpg"))
            .build()
            .expect("Could not create sink element.");

        pipeline.add_many(&[&encoder, &sink]).unwrap();
        gst::Element::link_many(&[&encoder, &sink]).expect("Elements could not be linked.");
        Ok(Self { sink: encoder })
    }
}
