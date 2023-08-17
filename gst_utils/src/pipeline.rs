use gst::prelude::*;
pub mod comp;
pub mod sink;
pub mod src;
use log::{debug, error, info};

#[allow(unused)]
pub struct Pipeline {
    pipeline: gst::Pipeline,
}

impl Pipeline {
    pub fn new() -> Result<Self, glib::Error> {
        gst::init()?;
        gst::debug_set_default_threshold(gst::DebugLevel::Error);
        let pipeline = gst::Pipeline::with_name("my-pipeline");
        pipeline.set_state(gst::State::Paused).unwrap();
        Ok(Self { pipeline: pipeline })
    }

    pub fn get_pipeline(&self) -> &gst::Pipeline {
        &self.pipeline
    }

    pub fn add_elements(&self, elements: &[&gst::Element]) -> Result<(), glib::BoolError> {
        let _ = &self.pipeline.add_many(elements)?;
        Ok(())
    }

    pub fn run(&self) -> Result<(), glib::Error> {
        self.pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set the pipeline to the `Playing` state");

        // Wait until error or EOS
        let bus = self.pipeline.bus().unwrap();
        for msg in bus.iter_timed(gst::ClockTime::NONE) {
            use gst::MessageView;

            match msg.view() {
                MessageView::Error(err) => {
                    error!(
                        "Error received from element {:?} {}",
                        err.src().map(|s| s.path_string()),
                        err.error()
                    );
                    debug!("Debugging information: {:?}", err.debug());
                    break;
                }
                MessageView::StateChanged(state_changed) => {
                    if state_changed
                        .src()
                        .map(|s| s == &self.pipeline)
                        .unwrap_or(false)
                    {
                        info!(
                            "Pipeline state changed from {:?} to {:?}",
                            state_changed.old(),
                            state_changed.current()
                        );
                    }
                }
                MessageView::Eos(..) => break,
                _ => (),
            }
        }
        self.pipeline
            .set_state(gst::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");

        Ok(())
    }
}
