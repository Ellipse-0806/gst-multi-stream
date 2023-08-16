use gst::prelude::*;

pub struct MultiStreamPL {
    compositor: gst::Element,
    converter: gst::Element,
}

impl MultiStreamPL {
    pub fn new(pipeline: &gst::Pipeline) -> Result<Self, glib::BoolError> {
        let compositor = gst::ElementFactory::make("compositor")
            .name("compositor")
            .build()
            .expect("Could not create compositor element.");

        let converter = gst::ElementFactory::make("videoconvert")
            .name("converter")
            .build()
            .expect("Could not create converter element.");

        pipeline.add_many(&[&compositor, &converter]).unwrap();

        Ok(Self {
            compositor: compositor,
            converter: converter,
        })
    }

    pub fn link_pad(
        &self,
        xpos: &i32,
        ypos: &i32,
        width: &i32,
        height: &i32,
        pad: &gst::Pad,
    ) -> Result<(), glib::BoolError> {
        let comp_sinkpad_temp = self.compositor.pad_template("sink_%u").unwrap();
        let comp_sinkpad = self
            .compositor
            .request_pad(&comp_sinkpad_temp, None, None)
            .unwrap();

        comp_sinkpad.set_properties(&[
            ("xpos", xpos as &dyn glib::ToValue),
            ("ypos", ypos as &dyn glib::ToValue),
            ("width", width as &dyn glib::ToValue),
            ("height", height as &dyn glib::ToValue),
        ]);

        pad.link(&comp_sinkpad)
            .expect("Elements could not be linked.");

        Ok(())
    }

    pub fn link(&self, element: &gst::Element) -> Result<(), glib::BoolError> {
        gst::Element::link_many(&[&self.compositor, &self.converter, element]).unwrap();
        Ok(())
    }
}
