use gst::prelude::*;

pub struct SourcePad {
    src_pad: gst::Pad,
}

impl SourcePad {
    pub fn new(
        uri: &String,
        pipeline: &gst::Pipeline,
        uniq_number: usize,
    ) -> Result<Self, glib::BoolError> {
        let caps = gst::Caps::builder("video/x-raw").build();

        let src = gst::ElementFactory::make("uridecodebin")
            .name(format!("src_{uniq_number}"))
            .property("uri", uri)
            .property("caps", &caps)
            .build()
            .expect("Could not create uridecodebin element.");

        let videoconvert = gst::ElementFactory::make("videoconvert")
            .name(format!("conv_{uniq_number}"))
            .build()
            .expect("Could not create videoconvert element.");

        pipeline.add_many(&[&src, &videoconvert]).unwrap();

        let videoconvert_weak = videoconvert.downgrade();
        src.connect_pad_added(move |src, src_pad| {
            println!("Received new pad {} from {}", src_pad.name(), src.name());
            let videoconvert = match videoconvert_weak.upgrade() {
                Some(videoconvert) => videoconvert,
                None => return,
            };

            src.downcast_ref::<gst::Bin>()
                .unwrap()
                .debug_to_dot_file_with_ts(gst::DebugGraphDetails::all(), "pad-added");

            let sink_pad = videoconvert
                .static_pad("sink")
                .expect("Failed to get static sink pad from convert");
            if sink_pad.is_linked() {
                println!("We are already linked. Ignoring.");
                return;
            }
            let res = src_pad.link(&sink_pad);
            if res.is_err() {
                println!("Type is video/x-raw but link failed.");
            } else {
                println!("Link succeeded (type video/x-raw).");
            }
        });

        gst::Element::link_many(&[&videoconvert]).unwrap();
        Ok(Self {
            src_pad: videoconvert.static_pad("src").unwrap(),
        })
    }

    pub fn get_src_pad(&self) -> &gst::Pad {
        &self.src_pad
    }
}
