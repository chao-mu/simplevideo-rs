#![warn(missing_docs)]
#![cfg_attr(feature = "strict", deny(warnings))]

//! A simple library for playback of videos and live access to the underlying
//! decoded bytes in each frame. The backend is gstreamer.
//!
//! # Examples
//!
//! ```
//! // Creates a video struct to control playback. Defaults to RGBA.
//! let video = simple_video::VideoBuilder::new("test-data/test.webm").load().unwrap();
//!
//! // You must start the video before continuing.
//! video.play();
//!
//! let bytes : Vec<u8> = video.read_frame().unwrap();
//! ```

pub mod formats;
pub use formats::*;

use failure;

use gst::prelude::*;
use gstreamer as gst;
use gstreamer_app as gst_app;

use std::sync::mpsc;
use std::thread;

/// Represents the result of attempting to read a frame
pub type FrameResult = Result<Vec<u8>, VideoError>;

/// Represents an error during loading or playback of video.
#[derive(failure::Fail, Debug)]
pub enum VideoError {
    /// There are no more frames to be read.
    #[fail(display = "No more frames available")]
    NoMore(),
    /// There has been a misc. failure during playback or attempt thereof
    #[fail(display = "{}", _0)]
    Failure(String),
    /// Reading from one of our internal channels failed.
    #[fail(display = "Failure to receive frame from channel")]
    RecvError(),
}

enum ControlChange {
    Play(),
    Pause(),
    Failure(String),
    Done(),
}

fn make_element(element_type: &str) -> gst::Element {
    gst::ElementFactory::make(element_type, None).expect("unable to create gst element")
}

fn on_new_sample(
    appsink: &gst_app::AppSink,
    out: &mpsc::SyncSender<FrameResult>,
) -> Result<gst::FlowSuccess, gst::FlowError> {
    // Pull the sample in question out of the appsink's buffer.
    let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;
    let buffer = sample.get_buffer().ok_or_else(|| {
        gst::gst_element_error!(
            appsink,
            gst::ResourceError::Failed,
            ("Failed to get buffer from appsink")
        );

        gst::FlowError::Error
    })?;

    let mut bytes = vec![0; buffer.get_size()];
    match buffer.copy_to_slice(0, bytes.as_mut_slice()) {
        Ok(_) => {}
        Err(_) => {
            gst::gst_element_error!(
                appsink,
                gst::ResourceError::Failed,
                ("Failed to copy frame to slice")
            );

            return Err(gst::FlowError::Error);
        }
    };

    let res = out.send(Ok(bytes)).map_or_else(
        |_e| {
            gst::gst_element_error!(
                appsink,
                gst::ResourceError::Failed,
                ("Failed to send frame")
            );
            Err(gst::FlowError::Error)
        },
        |_| Ok(gst::FlowSuccess::Ok),
    );

    res
}

fn setup_appsink(
    format: PixelFormat,
    out: mpsc::SyncSender<FrameResult>,
    appsink: &gst_app::AppSink,
) {
    appsink.set_caps(Some(&gst::Caps::new_simple(
        "video/x-raw",
        &[("format", &format.to_gst_format())],
    )));

    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::new()
            // Add a handler to the "new-sample" signal.
            .new_sample(move |appsink| on_new_sample(&appsink, &out))
            .build(),
    );
}

/// Video represents a video being played and is built with VideoBuilder.
/// # Examples
///
/// ```
/// // Creates a video struct to control playback. Defaults to RGBA.
/// let video = simple_video::VideoBuilder::new("test-data/test.webm").load().unwrap();
///
/// // Play the video making frames accessible.
/// video.play();
///
/// // Block until there is a video frame and read it.
/// let frame : Vec<u8> = video.read_frame().unwrap();
///
/// // You can then pause the video
/// video.pause();
///
/// // You must close the video before exiting or you will receive
/// // gstreamer warnings.
/// video.close();
/// ```
pub struct Video {
    frame_receiver: mpsc::Receiver<FrameResult>,
    control_sender: mpsc::Sender<ControlChange>,
}

impl Video {
    /// read_frame blocks until a frame is read or an error occurs.
    pub fn read_frame(&self) -> FrameResult {
        match self.frame_receiver.recv() {
            Err(mpsc::RecvError) => Err(VideoError::RecvError()),
            Ok(res) => res,
        }
    }

    /// A non-blocking version of read_frame. Will return None if there
    /// are no frames to be read.
    pub fn try_read_frame(&self) -> Option<FrameResult> {
        let mut res_opt = None;
        loop {
            let res = match self.frame_receiver.try_recv() {
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => Err(VideoError::RecvError()),
                Ok(res) => res,
            };

            res_opt = Some(res);
        }

        res_opt
    }

    /// Starts playback of video.
    pub fn play(&self) {
        self.control_sender.send(ControlChange::Play()).unwrap();
    }

    /// Pauses playback of video.
    pub fn pause(&self) {
        self.control_sender.send(ControlChange::Pause()).unwrap();
    }

    /// Closes video. There's no going back.
    pub fn close(&self) {
        self.control_sender.send(ControlChange::Done()).unwrap();
    }
}

/// Used to build a video and control what pixel format you
/// are provided
///
/// # Examples
///
/// ```
/// let video = simple_video::VideoBuilder::new("test-data/test.webm")
///     .desired_format(simple_video::PixelFormat::Bgra)
///     .load();
/// ```
pub struct VideoBuilder {
    path: String,
    pixel_format: PixelFormat,
}

impl VideoBuilder {
    /// Instantiates a new VideoBuilder with which you can
    /// build a video. The default pixel format is RGBA.
    pub fn new(path: &str) -> VideoBuilder {
        VideoBuilder {
            path: path.to_string(),
            pixel_format: PixelFormat::Rgba,
        }
    }

    /// Set the format that video frames will be converted to.
    pub fn desired_format(&mut self, format: PixelFormat) -> &mut Self {
        self.pixel_format = format;
        self
    }

    /// Loads an instance of the video.
    pub fn load(&self) -> Result<Video, VideoError> {
        gst::init().unwrap();

        let (frame_sender, frame_receiver): (
            mpsc::SyncSender<FrameResult>,
            mpsc::Receiver<FrameResult>,
        ) = mpsc::sync_channel(10);

        let (control_sender, control_receiver): (
            mpsc::Sender<ControlChange>,
            mpsc::Receiver<ControlChange>,
        ) = mpsc::channel();

        let pipeline = gst::Pipeline::new(None);

        let src = make_element("filesrc");
        let decodebin = make_element("decodebin");

        src.set_property("location", &self.path).map_err(|_| {
            VideoError::Failure(
                "Unable to set location property of src gstreamer element".to_string(),
            )
        })?;

        pipeline.add_many(&[&src, &decodebin]).map_err(|_| {
            VideoError::Failure("Failed to link gstreamer elements to pipeline".to_string())
        })?;

        gst::Element::link_many(&[&src, &decodebin])
            .map_err(|_| VideoError::Failure("Failed to link gstreamer elements".to_string()))?;

        let pipeline_weak = pipeline.downgrade();
        let pixel_format = self.pixel_format.clone();

        {
            let frame_sender = frame_sender.clone();
            decodebin.connect_pad_added(move |dbin, src_pad| {
                let pipeline = match pipeline_weak.upgrade() {
                    Some(pipeline) => pipeline,
                    None => return,
                };

                let (is_video, is_audio) = match src_pad.get_current_caps() {
                    Some(caps) => match caps.get_structure(0) {
                        Some(s) => {
                            let name = s.get_name();

                            (name.starts_with("video/"), name.starts_with("audio/"))
                        }
                        None => (false, false),
                    },
                    None => (false, false),
                };

                let insert_sink = |is_video, is_audio| -> Result<(), failure::Error> {
                    if is_video {
                        let queue = make_element("queue");
                        let convert = make_element("videoconvert");
                        let scale = make_element("videoscale");
                        let sink = make_element("appsink");

                        let elements = &[&queue, &convert, &scale, &sink];
                        pipeline.add_many(elements)?;
                        gst::Element::link_many(elements)?;

                        // Make sure elements have same state as pipeline
                        for e in elements {
                            e.sync_state_with_parent()?;
                        }

                        setup_appsink(
                            pixel_format,
                            frame_sender.clone(),
                            &sink
                                .dynamic_cast::<gst_app::AppSink>()
                                .expect("Sink element is expected to be an appsink!"),
                        );

                        // Get the queue element's sink pad and link the decodebin's newly created src pad for the audio stream to it.
                        let sink_pad = queue.get_static_pad("sink").expect("queue has no sinkpad");
                        src_pad.link(&sink_pad)?;
                    } else if is_audio {
                        let queue = make_element("queue");
                        let convert = make_element("audioconvert");
                        let resample = make_element("audioresample");
                        let sink = make_element("autoaudiosink");

                        let elements = &[&queue, &convert, &resample, &sink];
                        pipeline.add_many(elements)?;
                        gst::Element::link_many(elements)?;

                        // Make sure elements have same state as pipeline
                        for e in elements {
                            e.sync_state_with_parent()?;
                        }

                        // Get the queue element's sink pad and link the decodebin's newly created
                        // src pad for the audio stream to it.
                        let sink_pad = queue.get_static_pad("sink").expect("queue has no sinkpad");
                        src_pad.link(&sink_pad)?;
                    }

                    Ok(())
                };

                if let Err(err) = insert_sink(is_video, is_audio) {
                    gst::gst_element_error!(
                        dbin,
                        gst::LibraryError::Failed,
                        ("Failed to insert sink"),
                        ["{}", err]
                    );
                }
            });
        }

        let bus = pipeline
            .get_bus()
            .expect("Pipeline without bus. Shouldn't happen!");

        {
            let control_sender = control_sender.clone();
            thread::spawn(move || {
                // This code iterates over all messages that are sent across our pipeline's bus.
                // We can send better error information using bus messages from our callbacks.
                for msg in bus.iter_timed(gst::CLOCK_TIME_NONE) {
                    match msg.view() {
                        gst::MessageView::Eos(..) => {
                            let _ = control_sender.send(ControlChange::Done());
                            let _ = frame_sender.send(Err(VideoError::NoMore()));
                            break;
                        }
                        gst::MessageView::Error(err) => {
                            let _ = control_sender
                                .send(ControlChange::Failure(err.get_error().to_string()));
                            let _ = control_sender.send(ControlChange::Done());
                            let _ = frame_sender
                                .send(Err(VideoError::Failure(err.get_error().to_string())));
                            break;
                        }
                        gst::MessageView::StateChanged(..) => {}
                        _ => (),
                    };
                }
            });
        }

        thread::spawn(move || {
            for msg in control_receiver {
                match msg {
                    ControlChange::Play() => pipeline.set_state(gst::State::Playing).unwrap(),
                    ControlChange::Pause() => pipeline.set_state(gst::State::Paused).unwrap(),
                    ControlChange::Failure(..) => break,
                    ControlChange::Done() => {
                        pipeline.set_state(gst::State::Null).unwrap();
                        break;
                    }
                };
            }
        });

        let vid = Video {
            frame_receiver,
            control_sender,
        };

        Ok(vid)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
