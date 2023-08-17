use config_parser;
use gst_utils::{
    debug_utils,
    pipeline::{comp::MultiStreamPL, sink::SinkElement, src::uri::SourcePad, Pipeline},
};

fn main() {
    let config = config_parser::load().expect("Could not loading setting file.");
    let sources = config.get_sources();
    debug_utils::init_debug_utils();
    let pipeline = Pipeline::new().unwrap();
    let multi_stream = MultiStreamPL::new(pipeline.get_pipeline()).unwrap();
    let sink = SinkElement::init_fake(pipeline.get_pipeline()).unwrap();
    multi_stream.link(sink.get_element()).unwrap();

    for (i, source) in sources.iter().enumerate() {
        let src_pad = SourcePad::new(source.get_uri(), pipeline.get_pipeline(), i).unwrap();

        multi_stream
            .link_pad(
                source.get_x_pos(),
                source.get_y_pos(),
                source.get_width(),
                source.get_height(),
                src_pad.get_src_pad(),
            )
            .unwrap();
    }
    pipeline.run().unwrap();
    debug_utils::enable_debug_utils(pipeline.get_pipeline());
}
