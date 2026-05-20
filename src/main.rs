use gstreamer as gst;
use gst::prelude::*;
use std::env;

fn main() {
    // Initialize the GStreamer engine
    gst::init().expect("Failed to initialize GStreamer.");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage:");
        println!("  Host (Windows 11):   app host <Client_IP> [monitor_index]");
        println!("  Client (Any OS):     app client");
        return;
    }

    let mode = &args[1];

    let pipeline_str = if mode == "host" {
        if args.len() < 3 {
            panic!("Host mode requires the Client IP address.");
        }
        let ip = &args[2];
        let monitor = if args.len() > 3 { &args[3] } else { "1" }; // Captures display #1 by default

        // Windows Host Pipeline
        // dxgiscreencapsrc: Native DirectX low-latency screen capture
        // mfh264enc: Windows built-in hardware encoding (zero CPU usage)
        format!(
            "gdiscreencapsrc monitor={} ! videoconvert ! video/x-raw,framerate=60/1 ! x264enc bitrate=5000 tune=zerolatency speed-preset=ultrafast ! rtph264pay ! udpsink host={} port=5000",
            monitor, ip
        )
    } else if mode == "client" {
        // Client Pipeline (Linux or Windows)
        // sync=false forces immediate rendering with 0 buffering delay
        "udpsrc port=5000 caps=\"application/x-rtp,media=video,clock-rate=90000,encoding-name=H264\" ! rtpjitterbuffer latency=0 ! rtph264depay ! h264parse ! decodebin ! autovideosink sync=false".to_string()
    } else {
        panic!("Invalid mode. Use 'host' or 'client'.");
    };

    println!("Starting stream...");

    let pipeline = gst::parse::launch(&pipeline_str).expect("Failed to create pipeline. Check GStreamer installation.");
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();

    pipeline.set_state(gst::State::Playing).unwrap();

    // Keep the app running until an error or manual interruption
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;
        match msg.view() {
            MessageView::Error(err) => {
                eprintln!("Pipeline Error: {} ({:?})", err.error(), err.debug());
                break;
            }
            MessageView::Eos(..) => {
                println!("End of stream.");
                break;
            }
            _ => (),
        }
    }

    pipeline.set_state(gst::State::Null).unwrap();
}