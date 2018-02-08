extern crate divans;
#[cfg(feature="no-stdlib")]
fn main() {
    panic!("For no-stdlib examples please see the tests")
}
#[cfg(not(feature="no-stdlib"))]
fn main() {
    use std::io;
    let stdout = &mut io::stdout();
    {
        use std::io::{Read, Write};
        let mut writer = divans::DivansBrotliHybridCompressorWriter::new(
            stdout,
            divans::DivansCompressorOptions{
                basic: divans::DivansCompressorBasicOptions {
                    literal_adaptation:None, // should we override how fast the cdfs converge for literals?
                    window_size:Some(22), // log 2 of the window size
                    lgblock:None, // should we override how often metablocks are created in brotli
                    dynamic_context_mixing:Some(2), // if we want to mix together the stride prediction and the context map
                    prior_depth:None,
                    force_stride_value: divans::StrideSelection::UseBrotliRec, // if we should use brotli to decide on the stride
                    use_context_map:true, // whether we should use the brotli context map in addition to the last 8 bits of each byte as a prior
                },
                use_brotli:divans::BrotliCompressionSetting::default(), // ignored
                quality:Some(11), // the quality of brotli commands
                stride_detection_quality: Some(1),
            },
            4096, // internal buffer size
        );
        let mut buf = [0u8; 4096];
        loop {
            match io::stdin().read(&mut buf[..]) {
                Err(e) => {
                    if let io::ErrorKind::Interrupted = e.kind() {
                        continue;
                    }
                    panic!(e);
                }
                Ok(size) => {
                    if size == 0 {
                        match writer.flush() {
                            Err(e) => {
                                if let io::ErrorKind::Interrupted = e.kind() {
                                    continue;
                                }
                                panic!(e)
                            }
                            Ok(_) => break,
                        }
                    }
                    match writer.write_all(&buf[..size]) {
                        Err(e) => panic!(e),
                        Ok(_) => {},
                    }
                }
            }
        }
    }
}
